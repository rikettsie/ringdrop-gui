mod commands;
mod config;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new(config::read_daemon_port()))
        .invoke_handler(tauri::generate_handler![commands::daemon_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
