#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tracing::Level;

mod agent;
mod agentmgr;
mod cmds;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let agents = agentmgr::AgentMgr::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::agent_mgr_cache,
            cmds::add_pf_agent,
            cmds::add_gate_agent,
            cmds::pf_create_player,
            cmds::pf_refresh_players,
            cmds::pf_use_player,
            cmds::gate_send,
            cmds::gate_recv,
            cmds::gate_stop,
            cmds::listen_gate_recv,
            cmds::history_send,
            cmds::history_recv,
        ])
        .manage(agents)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
