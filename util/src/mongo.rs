use std::time::Duration;

use mongodb::options::{ClientOptions, ServerAddress};

use crate::gconf::ConfigDB;

pub async fn build_db(cfg: ConfigDB) -> anyhow::Result<mongodb::Database> {
    Ok(mongodb::Client::with_options(
        ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp {
                host: cfg.host,
                port: Some(cfg.port),
            }])
            .app_name(cfg.db_name.clone())
            .connect_timeout(Duration::from_secs(5))
            .default_database(cfg.db_name)
            .credential(
                mongodb::options::Credential::builder()
                    .username(cfg.user)
                    .password(cfg.password)
                    .build(),
            )
            .build(),
    )
    .map_err(anyhow::Error::from)?
    .default_database()
    .unwrap())
}
