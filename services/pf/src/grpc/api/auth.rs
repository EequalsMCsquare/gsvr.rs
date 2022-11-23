use axum::async_trait;
use spb::auth::{auth_service_server::AuthService, VerifyTokenAck, VerifyTokenReq};
use util::Jwt;

use crate::models;
#[derive(Clone)]
pub struct AuthSvc {
    pub jwt: Jwt<models::Claim>,
}
impl AuthSvc {
    pub fn new(jwt: Jwt<models::Claim>) -> Self {
        Self { jwt }
    }
}

#[async_trait]
impl AuthService for AuthSvc {
    async fn verify_token(
        &self,
        request: tonic::Request<VerifyTokenReq>,
    ) -> Result<tonic::Response<VerifyTokenAck>, tonic::Status> {
        let request = request.into_inner();
        match self.jwt.decode(&request.token) {
            Ok(claim) => Ok(tonic::Response::new(VerifyTokenAck {
                err_code: spb::ErrCode::Success as i32,
                id: claim.id,
            })),
            Err(err) => Ok(tonic::Response::new(VerifyTokenAck {
                err_code: spb::ErrCode::InvToken as i32,
                id: Default::default(),
            })),
        }
    }
}
