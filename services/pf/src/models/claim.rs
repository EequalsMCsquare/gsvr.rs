use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};
use serde::{Deserialize, Serialize};
use util::Jwt;

use super::PhantomAck;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub sub: String,
    pub exp: usize,
}

impl Claim {
    pub fn new(id: i64, sub: String, duration: time::Duration) -> Self {
        Self {
            id,
            sub,
            exp: (time::OffsetDateTime::now_utc() + duration).unix_timestamp() as usize,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claim
where
    B: Send,
{
    type Rejection = super::ApiResult<PhantomAck>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| {
                    tracing::error!("fail to get token from header. {}", err);
                    super::ApiResult::UnAuth
                })?;
        tracing::debug!("token: {:?}", token);
        let Extension(jwt): Extension<Jwt<Claim>> =
            Extension::from_request(req).await.map_err(|err| {
                tracing::error!("fail to get Extension<Jwt<Claim>>. {}", err);
                super::ApiResult::Internal(None)
            })?;
        jwt.decode(token.token()).map_err(|err| {
            tracing::error!("jwt decode error. {}", err);
            super::ApiResult::InvToken
        })
    }
}
