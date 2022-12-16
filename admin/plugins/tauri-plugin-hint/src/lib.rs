use tauri::{Runtime, plugin::{TauriPlugin, Builder}, Manager};

mod hint;
mod cmds;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("hint")
    .setup(|app_handle|{
      app_handle.manage(hint::Hint::new());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cmds::get_hint
    ])
    .build()
}