use core::time;

use serde::Deserialize;
#[derive(Deserialize, Debug, Default, Clone)]
pub struct ConfigLog {
    pub level: Option<String>,
    pub output: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ConfigDB {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: Option<String>,
    pub max_conn: Option<u32>,
    pub min_conn: Option<u32>,
    pub idle_timeout: Option<time::Duration>
}

#[derive(Deserialize, Debug)]
pub struct ConfigMQ {
    pub chanbuf: usize,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct ConfigJwt {
    pub algorithm: String,
    pub encode_key: String,
    pub decode_key: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub enum Env {
    #[default]
    Local,
    Dev,
    Qa,
    Prod,
}

impl Into<&'static str> for Env {
    fn into(self) -> &'static str {
        match self {
            Env::Local => "local",
            Env::Dev => "dev",
            Env::Qa => "qa",
            Env::Prod => "prod",
        }
    }
}

impl Into<String> for Env {
    fn into(self) -> String {
        match self {
            Env::Local => "local".to_string(),
            Env::Dev => "dev".to_string(),
            Env::Qa => "qa".to_string(),
            Env::Prod => "prod".to_string(),
        }
    }
}
