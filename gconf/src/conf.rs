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
