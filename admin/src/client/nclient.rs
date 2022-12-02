use crate::cmd::client::run_game_client;

use super::{
    gclient::{GClient, GClientBuilder},
    misc::{ClientInfo, Player},
    pfclient::PfClient,
};
use anyhow::anyhow;

// admin normal client, login with username and password
pub struct NClient {
    username: String,
    password: String,
    token: Option<String>,
    pfclient: PfClient,
    gate: String,
    _gclient: Option<GClient>,
}

impl NClient {
    pub fn new(
        pfclient: PfClient,
        gate: String,
        username: String,
        password: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            username,
            password,
            token: None,
            pfclient,
            gate,
            _gclient: None,
        })
    }

    pub async fn authorize(&mut self) -> anyhow::Result<()> {
        self.token = self
            .pfclient
            .auth_account(&self.username, &self.password)
            .await?
            .into();
        Ok(())
    }

    pub async fn list_players(&mut self) -> anyhow::Result<Vec<Player>> {
        if self.token.is_none() {
            self.authorize().await?;
        }
        self.pfclient
            .list_players(&self.token.as_ref().unwrap())
            .await
    }

    pub async fn create_player(&mut self, name: &str) -> anyhow::Result<Player> {
        if self.token.is_none() {
            self.authorize().await?;
        }
        self.pfclient
            .create_player(&self.token.as_ref().unwrap(), name)
            .await
    }

    pub async fn use_player(&mut self, id: i64) -> anyhow::Result<()> {
        if self.token.is_none() {
            self.authorize().await?;
        }
        let players = self
            .pfclient
            .list_players(&self.token.as_ref().unwrap())
            .await?;
        match players.into_iter().find(|p| p.id == id) {
            Some(player) => {
                let gclient = GClientBuilder::new()
                    .gate(&self.gate)
                    .info(ClientInfo::Normal {
                        player_id: id,
                        token: self.token.as_ref().unwrap().clone(),
                    })
                    .build()
                    .await?;
                run_game_client(gclient, format!("[{}/{}]", self.username, player.name)).await?;
                Ok(())
            }
            None => Err(anyhow!("player-{} not found", id)),
        }
    }
}
