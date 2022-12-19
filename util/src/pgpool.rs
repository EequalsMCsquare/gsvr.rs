use sqlx::postgres::PgPoolOptions;

use crate::gconf::ConfigDB;

pub async fn build(cfg: ConfigDB) -> Result<sqlx::PgPool, sqlx::Error> {
    let dbconn_str = format!(
        "postres://{user}:{password}@{host}:{port}/{database}",
        user = cfg.user,
        password = cfg.password,
        host = cfg.host,
        port = cfg.port,
        database = cfg.db_name.expect("database name must be provided")
    );
    tracing::info!("database connect string: {}", dbconn_str);
    PgPoolOptions::new()
        .max_connections(cfg.max_conn.unwrap_or(10))
        .min_connections(cfg.min_conn.unwrap_or(1))
        .idle_timeout(cfg.idle_timeout.map(Into::into))
        .connect(&dbconn_str)
        .await
}
