use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, str::FromStr};
use util::{jwt, Password};

mod conf;
mod grpc;
mod http;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = conf::Config::parse("etc/pf")?;
    util::init_logger(c.log);
    tracing::debug!("logger init complete");
    let dbconn_str = format!(
        "postres://{user}:{password}@{host}:{port}/{database}",
        user = c.database.user,
        password = c.database.password,
        host = c.database.host,
        port = c.database.port,
        database = c.database.db_name.expect("database name must be provided")
    );
    tracing::debug!("database connect string: {}", dbconn_str);
    // database
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&dbconn_str)
        .await?;
    tracing::info!("database connect success");
    // jwt
    let jwt = jwt::Jwt::<models::Claim>::from_config(&c.jwt)?;
    // password
    let password = Password::new();
    // HTTP Server
    let app = http::make_app()
        .layer(axum::Extension(db.clone()))
        .layer(axum::Extension(jwt.clone()))
        .layer(axum::Extension(password.clone()));
    let http_addr = format!("0.0.0.0:{}", c.http_port);
    let http_graceful = axum::Server::bind(
        &SocketAddr::from_str(&http_addr)
            .expect("fail to parse http server address"),
    )
    .serve(app.into_make_service())
    .with_graceful_shutdown(async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!("listen ctrl_c event fail. {}", err)
        }
    });
    tracing::info!("http server listen on: {}", http_addr);
    // todo: GRPC Server

    // let http_join = tokio::spawn(http_graceful);
    http_graceful.await?;
    Ok(())
}
