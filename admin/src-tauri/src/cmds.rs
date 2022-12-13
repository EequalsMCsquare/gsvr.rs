use std::sync::Arc;

use serde::Serialize;
use tauri::State;
use tokio::net::TcpStream;
use tracing::instrument;

use crate::{
    agent::{AccountPlayer, GateAgent, PfAgent},
    agentmgr::{AgentMgr, FrontAgentMgr},
};

#[derive(Serialize)]
pub struct AddPfClientAck {
    username: String,
    players: Vec<AccountPlayer>,
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn agent_mgr_cache(clients: State<AgentMgr>) -> Result<FrontAgentMgr, String> {
    Ok(clients.to_front())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn add_pf_agent(
    clients: State<AgentMgr>,
    username: String,
    password: String,
) -> Result<AddPfClientAck, String> {
    let mut agent = PfAgent::new(
        Arc::new("192.168.1.6:8100".into()),
        username.clone(),
        password,
    )
    .map_err(|err| err.to_string())?;
    let players = agent.list_players().map_err(|e| e.to_string())?;
    clients.upsert_pf_agent(agent);
    Ok(AddPfClientAck { username, players })
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn add_gate_agent(clients: State<'_, AgentMgr>, pid: i64) -> Result<(), String> {
    // try find agent in cold
    let agent = if let Some(agent) = clients.remove_cold_agent(pid) {
        agent.run().unwrap()
    } else {
        let stream = TcpStream::connect("192.168.1.6:8001")
            .await
            .map_err(|err| err.to_string())?;
        let mut agent = GateAgent::new(pid, stream);
        agent.fast_login().await.map_err(|err| err.to_string())?;
        agent.run().unwrap()
    };
    clients.upsert_gate_agent(agent);
    Ok(())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn pf_create_player(
    clients: State<AgentMgr>,
    username: &str,
    name: &str,
) -> Result<AccountPlayer, String> {
    let Some(mut agent) = clients.get_pf_agent(username) else {
        return Err(format!("login with {} first", username));
    };
    agent.create_player(name).map_err(|err| err.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn pf_refresh_players(
    clients: State<AgentMgr>,
    username: &str,
) -> Result<Vec<AccountPlayer>, String> {
    let Some(mut agent) = clients.get_pf_agent(username) else {
        return Err(format!("login with {} first", username));
    };
    agent.list_players().map_err(|err| err.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn pf_use_player(
    clients: State<'_, AgentMgr>,
    username: &str,
    pid: i64,
) -> Result<(), String> {
    // find pf agent first
    let Some(pfagent) = clients.get_pf_agent(username) else {
        return Err(format!("login with {} first", username));
    };

    // try find agent in cold
    let agent = if let Some(agent) = clients.remove_cold_agent(pid) {
        agent.run().unwrap()
    } else {
        let stream = TcpStream::connect("192.168.1.6:8001")
            .await
            .map_err(|err| err.to_string())?;
        let mut agent = GateAgent::new(pid, stream);
        agent
            .login(&pfagent.token)
            .await
            .map_err(|err| err.to_string())?;
        agent.run().unwrap()
    };
    clients.upsert_gate_agent(agent);
    Ok(())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_send(
    clients: State<'_, AgentMgr>,
    pid: i64,
    msg: cspb::Registry,
) -> Result<(), String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    agent.send(msg).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn gate_recv(clients: State<'_, AgentMgr>, pid: i64) -> Result<cspb::Registry, String> {
    let Some(mut agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    agent
        .recv()
        .await
        .ok_or(format!("recv None from gate agent"))
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn history_send(
    clients: State<AgentMgr>,
    pid: i64,
    limit: usize,
    reverse: bool,
) -> Result<Vec<cspb::Registry>, String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let his = agent.history();
    Ok(his.get_send(limit, reverse))
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn history_recv(
    clients: State<AgentMgr>,
    pid: i64,
    limit: usize,
    reverse: bool,
) -> Result<Vec<cspb::Registry>, String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let his = agent.history();
    Ok(his.get_recv(limit, reverse))
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_stop(clients: State<'_, AgentMgr>, pid: i64) -> Result<(), String> {
    let Some(agent) = clients.remove_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let agent_inner = agent.stop().await.map_err(|e| e.to_string())?;
    clients.upsert_cold_agent(agent_inner);
    Ok(())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn listen_gate_recv(clients: State<'_, AgentMgr>, pid: i64) -> Result<(), String> {
    todo!()
}
