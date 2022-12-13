use anyhow::bail;
use serde::Deserialize;
use serde_json::json;

use super::misc::Player;

#[derive(Clone)]
pub struct PfClient {
    inner: reqwest::Client,
    pfaddr: String,
}

impl PfClient {
    pub fn new(pfaddr: String) -> Self {
        Self {
            inner: reqwest::ClientBuilder::new().build().unwrap(),
            pfaddr,
        }
    }
    pub async fn auth_account(&self, username: &str, password: &str) -> anyhow::Result<String> {
        let resp = self
            .inner
            .get(format!("{}/api/v1/account/auth", self.pfaddr))
            .json(&json!({
                "username": username,
                "password": password
            }))
            .send()
            .await?
            .json::<ApiReply<AuthAccountAck>>()
            .await?;
        if resp.code != 0 {
            bail!("Api ErrCode: {}. Message: {}", resp.code, resp.msg)
        } else {
            Ok(resp.data.unwrap().token)
        }
    }

    pub async fn create_player(&mut self, token: &str, name: &str) -> anyhow::Result<Player> {
        let resp: ApiReply<CreatePlayerAck> = self
            .inner
            .post(format!("{}/api/v1/player", self.pfaddr))
            .bearer_auth(token)
            .json(&serde_json::json!({ "name": name }))
            .send()
            .await?
            .json()
            .await?;
        if resp.code != 0 {
            bail!("Api ErrCode: {}. Message: {}", resp.code, resp.msg)
        } else {
            Ok(Player {
                id: resp.data.unwrap().player_id,
                name: name.to_owned(),
            })
        }
    }

    pub async fn list_players(&mut self, token: &str) -> anyhow::Result<Vec<Player>> {
        let resp: ApiReply<ListPlayersAck> = self
            .inner
            .get(format!("{}/api/v1/player", self.pfaddr))
            .bearer_auth(token)
            .send()
            .await?
            .json()
            .await?;
        if resp.code != 0 {
            bail!("Api ErrCode: {}. Message: {}", resp.code, resp.msg)
        } else {
            Ok(resp.data.unwrap().players)
        }
    }
}

#[derive(Deserialize)]
struct ApiReply<T> {
    code: i32,
    data: Option<T>,
    #[serde(default)]
    msg: String,
}

#[derive(Deserialize)]
struct AuthAccountAck {
    pub token: String,
}

#[derive(Deserialize)]
struct CreatePlayerAck {
    player_id: i64,
}

#[derive(Deserialize)]
struct ListPlayersAck {
    players: Vec<Player>,
}
