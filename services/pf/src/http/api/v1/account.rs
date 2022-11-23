use crate::models;

use axum::{Extension, Json};
use models::ApiResult;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Duration;
use util::{Jwt, Password};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ReqRegisterAccount {
    #[validate(length(min = 6, max = 64))]
    pub username: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
    #[validate(email)]
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl ReqRegisterAccount {
    pub fn validate_fields() -> &'static Lazy<Vec<&'static str>, fn() -> Vec<&'static str>> {
        static FIELDS: Lazy<Vec<&'static str>, fn() -> Vec<&'static str>> =
            Lazy::new(|| vec!["username", "password", "email"]);
        &FIELDS
    }
}

#[derive(Debug, Serialize)]
pub struct AckRegisterAccount {
    pub id: i64,
    pub username: String,
}

pub async fn register(
    Json(payload): Json<ReqRegisterAccount>,
    Extension(db): Extension<PgPool>,
    Extension(password): Extension<Password>,
) -> ApiResult<AckRegisterAccount> {
    if let Err(err) = payload.validate() {
        let err_map = err.errors();
        for &field in ReqRegisterAccount::validate_fields().iter() {
            if let Some(field_err) = err_map.get(field) {
                tracing::error!("validation error: {:?}", field_err);
                return ApiResult::BadParam(field.to_string());
            }
        }
        return ApiResult::Internal(None);
    }
    let hashed_password = match password.hash(&payload.password) {
        Ok(passwd) => passwd.to_string(),
        Err(err) => {
            tracing::error!("fail to hash password. {}", err);
            return ApiResult::Internal(None);
        }
    };
    match sqlx::query!(
        r#"INSERT INTO public.accounts (username, password, email, phone)
            VALUES($1, $2, $3, $4) RETURNING id"#,
        payload.username,
        hashed_password,
        payload.email,
        payload.phone
    )
    .fetch_one(&db)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(err) if err.constraint().is_some() => {
            ApiResult::DupIndex(err.constraint().unwrap().to_string())
        }
        _unknown => {
            tracing::error!("sqlx error: {}", _unknown);
            ApiResult::Internal(None)
        }
    }) {
        Ok(rec) => ApiResult::Ok(Some(AckRegisterAccount {
            id: rec.id,
            username: payload.username,
        })),
        Err(err) => err,
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReqAuthAccount {
    #[validate(length(min = 6, max = 64))]
    pub username: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

impl ReqAuthAccount {
    pub fn validate_fields() -> &'static Lazy<Vec<&'static str>, fn() -> Vec<&'static str>> {
        static FIELDS: Lazy<Vec<&'static str>, fn() -> Vec<&'static str>> =
            Lazy::new(|| vec!["username", "password"]);
        &FIELDS
    }
}

#[derive(Debug, Serialize)]
pub struct AckAuthAccount {
    pub token: String,
}

pub async fn auth(
    Json(payload): Json<ReqAuthAccount>,
    Extension(db): Extension<PgPool>,
    Extension(jwt): Extension<Jwt<models::Claim>>,
    Extension(password): Extension<Password>,
) -> ApiResult<AckAuthAccount> {
    // validate
    if let Err(err) = payload.validate() {
        let err_map = err.errors();
        for &field in ReqAuthAccount::validate_fields().iter() {
            if let Some(field_err) = err_map.get(field) {
                tracing::error!("validation error: {:?}", field_err);
                return ApiResult::BadParam(field.to_string());
            }
        }
        return ApiResult::Internal(None);
    }

    match sqlx::query!(
        r#"SELECT id, username, password as hashed_password FROM public.accounts 
            WHERE public.accounts.username = $1"#,
        payload.username
    )
    .fetch_one(&db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ApiResult::NoRecord(Some("account not exist".to_string())),
        _unknown => ApiResult::Internal(None),
    }) {
        Ok(row) => match password.verify(&payload.password, &row.hashed_password) {
            Ok(_) => match jwt.encode(models::Claim::new(row.id, row.username, Duration::hours(2)))
            {
                Ok(token) => ApiResult::Ok(Some(AckAuthAccount { token })),
                Err(err) => {
                    tracing::error!("fail to encode error. {}", err);
                    ApiResult::Internal(None)
                }
            },
            Err(err) => {
                tracing::error!("password verify fail: {}", err);
                ApiResult::Credential
            }
        },
        Err(err) => err,
    }
}
