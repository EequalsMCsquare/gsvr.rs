mod r#enum {
    include!("./spb.r#enum.rs");
}

pub mod auth {
    include!("./spb.auth.rs");
}

pub use r#enum::ErrCode;
pub use auth::auth_service_server::AuthServiceServer;
pub use auth::auth_service_client::AuthServiceClient;