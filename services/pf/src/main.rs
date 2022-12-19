use std::{net::SocketAddr, str::FromStr};
use util::{jwt, Password};

mod conf;
mod grpc;
mod http;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = conf::Config::parse("etc/pf")?;
    let _guard = util::logger::init(c.log);
    // database
    let db = util::pgpool::build(c.database).await?;
    tracing::info!("database connect success");
    // jwt
    let jwt = jwt::Jwt::<models::Claim>::build(c.jwt)?;
    // password
    let password = Password::new();
    // HTTP Server
    let app = http::make_app()
        .layer(axum::Extension(db.clone()))
        .layer(axum::Extension(jwt.clone()))
        .layer(axum::Extension(password.clone()));
    let http_addr = format!("0.0.0.0:{}", c.http_port);
    let http_future = axum::Server::bind(
        &SocketAddr::from_str(&http_addr).expect("fail to parse http server address"),
    )
    .serve(app.into_make_service())
    .with_graceful_shutdown(async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!("listen ctrl_c event fail. {}", err)
        }
    });
    tracing::info!("http server listen on: {}", http_addr);
    // GRPC Server
    let auth_svc = spb::AuthServiceServer::new(grpc::AuthSvc::new(jwt.clone()));
    let grpc_addr = format!("0.0.0.0:{}", c.grpc_port);
    let grpc_future = tonic::transport::Server::builder()
        .add_service(auth_svc)
        .serve_with_shutdown(grpc_addr.parse()?, async {
            if let Err(err) = tokio::signal::ctrl_c().await {
                tracing::error!("listen ctrl_c event fail. {}", err)
            }
        });
    tracing::info!("grpc server listen on: {}", grpc_addr);
    let (http_ret, grpc_ret) = tokio::join! {
        tokio::spawn(http_future),
        tokio::spawn(grpc_future)
    };
    http_ret??;
    grpc_ret??;
    Ok(())
}
