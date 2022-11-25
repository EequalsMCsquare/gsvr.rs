use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub last_login: Option<time::PrimitiveDateTime>,
    pub created_at: time::PrimitiveDateTime,
    pub updated_at: time::PrimitiveDateTime,
}
