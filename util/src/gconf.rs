use crate::custom_serde::StrDuration;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigEtcd {
    pub endpoints: Vec<String>,
    pub conn_timeout: StrDuration,
    pub request_timeout: StrDuration,
    pub keepalive_interval: StrDuration,
    pub keepalive_timeout: StrDuration,
    pub keepalive_idle: bool,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ConfigLog {
    pub level: Option<String>,
    pub output: Option<String>,
    pub enable_level: Option<bool>,
    pub enable_file: Option<bool>,
    pub enable_line: Option<bool>,
    pub enable_thread_name: Option<bool>,
    pub enable_thread_id: Option<bool>,
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
    pub idle_timeout: Option<StrDuration>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigMQ {
    pub chanbuf: usize,
    pub host: String,
    pub port: u16,

    #[serde(default = "ConfigMQ::default_conn_timeout")]
    pub conn_timeout: StrDuration,
    #[serde(default = "ConfigMQ::default_client_capacity")]
    pub client_capacity: usize,
    #[serde(default = "ConfigMQ::default_subscription_capacity")]
    pub subscription_capacity: usize,
    #[serde(default = "ConfigMQ::default_ping_interval")]
    pub ping_interval: StrDuration,
    #[serde(default = "ConfigMQ::default_flush_interval")]
    pub flush_interval: StrDuration,
    pub request_timeout: Option<StrDuration>,
}

impl ConfigMQ {
    fn default_conn_timeout() -> StrDuration {
        Duration::from_secs(5).into()
    }
    fn default_client_capacity() -> usize {
        128
    }
    fn default_subscription_capacity() -> usize {
        1024
    }
    fn default_ping_interval() -> StrDuration {
        time::Duration::minutes(1).try_into().unwrap()
    }
    fn default_flush_interval() -> StrDuration {
        time::Duration::milliseconds(100).try_into().unwrap()
    }
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
