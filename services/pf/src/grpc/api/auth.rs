use axum::async_trait;
use spb::auth::{gate_service_server::GateService, GateLoginReq, GateLoginAck};
use sqlx::PgPool;
pub struct GateServiceImpl {
    db: PgPool,
}

#[async_trait]
impl GateService for GateServiceImpl {
        async fn gate_login(
            &self,
            request: tonic::Request<GateLoginReq>,
        ) -> Result<tonic::Response<GateLoginAck>, tonic::Status> {
            todo!()
        }
}
