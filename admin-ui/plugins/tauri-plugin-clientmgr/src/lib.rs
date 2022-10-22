mod api;
mod error;
mod state;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("clientmgr")
        .invoke_handler(tauri::generate_handler![
			api::add_fclient,
			api::drop_fclient,
			api::fclient_request,
		])
        .setup(|app| {
            let (fclients, mut frx) = state::ClientMgr::<u64>::new();
            app.manage(fclients);
            let (nclients, mut nrx) = state::ClientMgr::<String>::new();
            app.manage(nclients);

            let app = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::select! {
                      f_ret = frx.recv() => {
                        if let Some(msg) = f_ret {
                            tracing::debug!("recv fscmsg: {:?}", msg);
                            if let Err(err) = &app.emit_all("recv_fscmsg", msg) {
                                tracing::error!("emit recv_fscmsg error. {}", err);
                            }
                        }
                      },
                      n_ret = nrx.recv() => {
                        if let Some(msg) = n_ret {
                            tracing::debug!("recv nscmsg: {:?}", msg);
                            if let Err(err) = &app.emit_all("recv_nscmsg", msg) {
                                tracing::error!("emit recv_nscmsg error. {}", err);
                            }
                        }
                      }
                    }
                }
            });
            Ok(())
        })
        .build()
}
