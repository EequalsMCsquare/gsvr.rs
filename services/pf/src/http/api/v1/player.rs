use crate::models::{ApiResult, Claim, Player};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct AckGetPlayers {
    players: Vec<Player>,
}

pub async fn get_players(
    claim: Claim,
    Extension(db): Extension<PgPool>,
) -> ApiResult<AckGetPlayers> {
    match sqlx::query_as!(
        Player,
        r#"
        SELECT id, name, "lastLogin" as last_login, 
            "createdAt" as created_at, "updatedAt" as updated_at 
            FROM public.players where public.players."accountId" = $1
        "#,
        claim.id
    )
    .fetch_all(&db)
    .await
    {
        Ok(players) => ApiResult::Ok(Some(AckGetPlayers { players })),
        Err(err) => {
            tracing::error!("{:?}", err);
            ApiResult::Internal(None)
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReqCreatePlayer {
    #[validate(length(min = 6, max = 16))]
    name: String,
}

#[derive(Debug, Serialize)]
pub struct AckCreatePlayer {
    player_id: i64,
}

pub async fn create_player(
    claim: Claim,
    Json(payload): Json<ReqCreatePlayer>,
    Extension(db): Extension<PgPool>,
) -> ApiResult<AckCreatePlayer> {
    match sqlx::query!(
        r#"INSERT INTO public.players ("accountId", id, name)
                SELECT $1 as "accountId", COUNT(1)+1 as id, $2 as name FROM
                public.players WHERE public.players."accountId" = $3
            RETURNING id
        "#,
        claim.id,
        payload.name,
        claim.id
    )
    .fetch_one(&db)
    .await
    {
        Ok(rec) => ApiResult::Ok(Some(AckCreatePlayer { player_id: rec.id })),
        Err(err) => {
            tracing::error!("{:?}", err);
            ApiResult::DupIndex("player.name".to_string())
        }
    }
}
