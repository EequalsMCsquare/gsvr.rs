#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() {
    util::init_logger(gconf::ConfigLog {
        output: Some(String::from("stdout")),
        level: Default::default(),
    });
    tracing::debug!("logger init");

    tauri::Builder::default()
        .plugin(tauri_plugin_clientmgr::init())
        .plugin(tauri_plugin_pbhint::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
