use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct PhantomAck;

#[repr(i32)]
pub enum ErrCode {
    Success = 0,
    Internal = 1,
    NoRecord = 2,
    DupIndex = 3,
    BadParam = 4,
    Credential = 5,
    UnAuth = 6,
    InvToken = 7,
}

#[allow(unused)]
#[derive(Serialize)]
pub enum ApiResult<T>
where
    T: Serialize,
{
    Ok(Option<T>),
    Internal(Option<String>),
    NoRecord(Option<String>),
    DupIndex(String),
    BadParam(String),
    Credential,
    UnAuth,
    InvToken,
}

impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::Ok(Some(data)) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::Success as i32,
                    "data": data
                })),
            )
                .into_response(),
            Self::Ok(None) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::Success as i32,
                })),
            )
                .into_response(),

            Self::Internal(msg) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::Internal as i32,
                    "msg": msg.unwrap_or_else(||"internal server error".to_string())
                })),
            )
                .into_response(),

            Self::NoRecord(msg) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::NoRecord as i32,
                    "msg": msg.unwrap_or_else(||"no record".to_string())
                })),
            )
                .into_response(),

            Self::DupIndex(field) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::DupIndex as i32,
                    "msg": format!("duplicated indexed field: {}", field),
                })),
            )
                .into_response(),
            Self::BadParam(field) => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::BadParam as i32,
                    "msg": field
                })),
            )
                .into_response(),
            Self::Credential => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::Credential as i32,
                    "msg": String::from("invalid credential")
                })),
            )
                .into_response(),
            Self::UnAuth => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::UnAuth as i32,
                    "msg": String::from("unauthorized")
                })),
            )
                .into_response(),
            Self::InvToken => (
                StatusCode::OK,
                Json(json!({
                    "code": ErrCode::InvToken as i32,
                    "msg": String::from("invalid token")
                })),
            )
                .into_response(),
        }
    }
}
