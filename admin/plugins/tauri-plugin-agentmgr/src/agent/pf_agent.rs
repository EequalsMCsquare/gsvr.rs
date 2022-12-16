use std::sync::Arc;

use anyhow::{bail, Ok};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct PfAgent {
    pub username: String,
    pub password: String,
    pub token: String,
    pfaddr: Arc<String>,

    inner: reqwest::blocking::Client,
    players: Vec<AccountPlayer>,
}

#[derive(Serialize)]
pub struct FrontPfAgent {
    pub _username: String,
    pub _players: Vec<AccountPlayer>,
}

impl PfAgent {
    pub fn new(pfaddr: Arc<String>, username: String, password: String) -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::new();
        let resp: ApiReply<AuthAck> = client
            .get(format!("{}/api/v1/account/auth", pfaddr))
            .json(&json!({
                "username": username,
                "password": password
            }))
            .send()?
            .json()?;
        if resp.code != 0 || resp.data.is_none() {
            bail!("{:?}", resp)
        }
        Ok(Self {
            username,
            password,
            pfaddr,
            inner: client,
            token: resp.data.unwrap().token,
            players: Vec::with_capacity(4),
        })
    }

    pub fn to_front(&self) -> FrontPfAgent {
        FrontPfAgent {
            _username: self.username.clone(),
            _players: self.players.clone(),
        }
    }

    pub fn create_player(&mut self, name: &str) -> anyhow::Result<AccountPlayer> {
        let resp: ApiReply<CreatePlayerAck> = self
            .inner
            .post(format!("{}/api/v1/player", self.pfaddr))
            .bearer_auth(&self.token)
            .json(&json!({ "name": name }))
            .send()?
            .json()?;
        if resp.code != 0 || resp.data.is_none() {
            bail!("{:?}", resp)
        }
        let ret = AccountPlayer {
            pid: resp.data.unwrap().player_id,
            name: name.to_owned(),
        };
        self.players.push(ret.clone());
        Ok(ret)
    }

    pub fn list_players(&mut self) -> anyhow::Result<Vec<AccountPlayer>> {
        let resp: ApiReply<ListPlayersAck> = self
            .inner
            .get(format!("{}/api/v1/player", self.pfaddr))
            .bearer_auth(&self.token)
            .send()?
            .json()?;
        if resp.code != 0 || resp.data.is_none() {
            bail!("{:?}", resp)
        }
        self.players = resp.data.unwrap().players;
        Ok(self.players.clone())
    }
}

#[derive(Deserialize, Debug)]
struct ApiReply<T> {
    code: i32,
    data: Option<T>,
    #[serde(default)]
    msg: String,
}

#[derive(Deserialize, Debug)]
struct AuthAck {
    token: String,
}

#[derive(Deserialize, Debug)]
struct CreatePlayerAck {
    player_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountPlayer {
    pub pid: i64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
struct ListPlayersAck {
    players: Vec<AccountPlayer>,
}
