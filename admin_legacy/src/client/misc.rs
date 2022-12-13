use async_trait::async_trait;
use serde::Deserialize;

#[derive(Clone)]
pub enum ClientInfo {
    Normal { player_id: i64, token: String },
    FastLogin { player_id: i64 },
}

#[async_trait]
pub trait AdminClient {
    async fn send(&mut self, msg: cspb::Registry) -> anyhow::Result<()>;
    async fn recv(&mut self) -> anyhow::Result<cspb::Registry>;
}

#[derive(Deserialize, Clone)]
pub struct Player {
    pub id: i64,
    pub name: String,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}
