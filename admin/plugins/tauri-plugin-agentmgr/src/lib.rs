use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod agent;
mod agentmgr;
mod cmds;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("agentmgr")
        .setup(|app_handle| {
            app_handle.manage(agentmgr::AgentMgr::<R>::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmds::agent_mgr_cache,
            cmds::add_pf_agent,
            cmds::add_gate_agent,
            cmds::pf_create_player,
            cmds::pf_refresh_players,
            cmds::pf_use_player,
            cmds::gate_send,
            cmds::gate_recv,
            cmds::gate_listen_recv,
            cmds::gate_unlisten_recv,
            cmds::gate_stop,
            cmds::listen_gate_recv,
            cmds::history_send,
            cmds::history_recv,
        ])
        .build()
}

