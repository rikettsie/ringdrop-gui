//! Tauri commands — thin wrappers around `DaemonClient::send`.
//!
//! Every command follows the same pattern:
//! ```text
//! state.client()  →  DaemonClient::send(Op::…, |event| window.emit("rd_event", event))
//! ```

use serde::Serialize;
use tauri::State;

use crate::state::AppState;

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DaemonStatus {
    pub running: bool,
    /// Port the daemon is (or should be) listening on, if configured.
    pub port: Option<u16>,
}

// ── Commands ──────────────────────────────────────────────────────────────────

/// Returns whether the ringdrop daemon is reachable and the configured port.
///
/// Called by `DaemonBadge` on a polling interval to update the connection
/// indicator. Safe to call when no config exists — returns `running: false`.
#[tauri::command]
pub async fn daemon_status(state: State<'_, AppState>) -> Result<DaemonStatus, String> {
    let running = match state.client() {
        None => false,
        Some(client) => client.is_running().await,
    };
    Ok(DaemonStatus {
        running,
        port: state.port,
    })
}
