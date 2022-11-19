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
}

#[derive(Deserialize, Debug)]
pub struct ConfigMQ {
    pub chanbuf: usize,
    pub host: String,
    pub port: u16,
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
