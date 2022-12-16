use std::sync::Arc;

use serde::Serialize;
use tauri::{Runtime, State, Window};
use tokio::net::TcpStream;
use tracing::instrument;

use super::{
    agent::{AccountPlayer, FrontHistoryData, GateAgent, PfAgent},
    agentmgr::{AgentMgr, FrontAgentMgr},
};

#[derive(Serialize)]
pub struct AddPfClientAck {
    username: String,
    players: Vec<AccountPlayer>,
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn agent_mgr_cache<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
) -> Result<FrontAgentMgr, String> {
    Ok(clients.to_front())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn add_pf_agent<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
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
pub async fn add_gate_agent<R: Runtime>(
    window: tauri::Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<(), String> {
    // try find agent in cold
    let agent = if let Some(agent) = clients.remove_cold_agent(pid) {
        agent.run().unwrap()
    } else {
        let stream = TcpStream::connect("192.168.1.6:8001")
            .await
            .map_err(|err| err.to_string())?;
        let mut agent = GateAgent::new(pid, stream, window);
        agent.fast_login().await.map_err(|err| err.to_string())?;
        agent.run().unwrap()
    };
    clients.upsert_gate_agent(agent);
    Ok(())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn pf_create_player<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
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
pub fn pf_refresh_players<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
    username: &str,
) -> Result<Vec<AccountPlayer>, String> {
    let Some(mut agent) = clients.get_pf_agent(username) else {
        return Err(format!("login with {} first", username));
    };
    agent.list_players().map_err(|err| err.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn pf_use_player<R: Runtime>(
    window: tauri::Window<R>,
    clients: State<'_, AgentMgr<R>>,
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
        let mut agent = GateAgent::new(pid, stream, window);
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
pub async fn gate_send<R: Runtime>(
    _window: Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
    msg: &str,
) -> Result<String, String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let msg = serde_json::from_str(msg).map_err(|err| err.to_string())?;
    let his = FrontHistoryData::new(&msg);
    let ret = serde_json::to_string(&his).map_err(|err| err.to_string())?;
    agent.send(msg).await.map_err(|err| err.to_string())?;
    Ok(ret)
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_recv<R: Runtime>(
    _window: Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<String, String> {
    let Some(mut agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let msg = agent
        .recv()
        .await
        .ok_or(format!("recv None from gate agent"))?;
    let his = FrontHistoryData::new(&msg);
    serde_json::to_string(&his).map_err(|err| err.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_listen_recv<R: Runtime>(
    _window: Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<(), String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    agent
        .listen_ack(format!("psc-{}", agent.pid))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_unlisten_recv<R: Runtime>(
    _window: Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<(), String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    agent.unlisten_ack().await.map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(clients))]
pub async fn gate_stop<R: Runtime>(
    _window: Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<(), String> {
    let Some(agent) = clients.remove_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let agent_inner = agent.stop().await.map_err(|e| e.to_string())?;
    clients.upsert_cold_agent(agent_inner);
    Ok(())
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn history_send<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
    pid: i64,
    limit: usize,
    reverse: bool,
) -> Result<String, String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let his = agent.history();
    Ok(his.get_send_json(limit, reverse))
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn history_recv<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<AgentMgr<R>>,
    pid: i64,
    limit: usize,
    reverse: bool,
) -> Result<String, String> {
    let Some(agent) = clients.get_gate_agent(pid) else {
        return Err(format!("gate login with {} first", pid));
    };
    let his = agent.history();
    Ok(his.get_recv_json(limit, reverse))
}

#[tauri::command]
#[instrument(skip(clients))]
pub fn listen_gate_recv<R: Runtime>(
    _window: tauri::Window<R>,
    clients: State<'_, AgentMgr<R>>,
    pid: i64,
) -> Result<(), String> {
    todo!()
}
