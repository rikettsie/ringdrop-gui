mod commands;
mod config;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new(config::read_daemon_port()))
        .invoke_handler(tauri::generate_handler![
            commands::daemon_status,
            commands::blob_list,
            commands::blob_import,
            commands::blob_remove,
            commands::receive,
            commands::node_id,
            commands::ring_list,
            commands::ring_create,
            commands::ring_members,
            commands::ring_add,
            commands::ring_remove,
            commands::peer_list,
            commands::peer_add,
            commands::peer_remove,
            commands::grant_list,
            commands::grant_grant,
            commands::grant_revoke,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
