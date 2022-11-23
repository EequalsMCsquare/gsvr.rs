use std::path::Path;

use anyhow::anyhow;
use config::File;
use util::gconf::{ConfigDB, ConfigLog, ConfigMQ, Env};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: ConfigLog,
    pub mq: ConfigMQ,
    pub database: ConfigDB,
    pub env: Env,
    pub sid: u16,
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
        let mut cfg: Config = cfg.try_deserialize().map_err(anyhow::Error::from)?;
        cfg.database.db_name = if let Some(db_name) = cfg.database.db_name {
            Some(
                db_name
                    .replace("${ENV}", &env)
                    .replace("${SID}", &cfg.sid.to_string()),
            )
        } else {
            Some(format!("GAME_{}_{}", env, cfg.sid))
        };
        Ok(cfg)
    }
}
