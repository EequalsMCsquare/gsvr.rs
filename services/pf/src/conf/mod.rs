use std::path::Path;

use anyhow::anyhow;
use config::File;
use gconf::{ConfigDB, ConfigJwt, ConfigLog, Env};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: ConfigLog,
    pub database: ConfigDB,
    pub jwt: ConfigJwt,
    pub env: Env,
    pub http_port: u16,
    pub grpc_port: u16,
}

impl Config {
    pub fn parse(dir: &str) -> anyhow::Result<Self> {
        let env = std::env::var("APP_ENV")
            .expect("APP_ENV is empty")
            .to_lowercase();
        let path = Path::new(dir)
            .join(&env)
            .to_str()
            .ok_or(anyhow!("fail to parse config file path"))?
            .to_owned();
        let s = config::Config::builder()
            .add_source(File::with_name(&format!("etc/default/{}", env)))
            .add_source(File::with_name(&path))
            .add_source(config::Environment::with_prefix("APP"));
        let cfg = if let Ok(envcfg) = std::env::var("APP_CFGDIR") {
            s.add_source(File::with_name(&envcfg)).build()
        } else {
            s.build()
        }?;
        cfg.try_deserialize().map_err(Into::into)
    }
}
