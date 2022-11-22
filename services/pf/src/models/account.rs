use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: i64,
    pub username: String, 
    pub password: String,
    pub email: String, 
    pub phone: String,
    pub last_login: time::Time,
    pub created_at: time::Time,
    pub updated_at: time::Time,
    pub deleted_at: time::Time
}