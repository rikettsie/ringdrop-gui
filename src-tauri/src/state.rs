//! Shared application state and IPC helpers injected into every Tauri command.

use ringdrop::daemon::{
    client::DaemonClient,
    protocol::{EventKind, Op},
};
use serde::de::DeserializeOwned;

/// Holds the daemon port read at startup from `~/.ringdrop/config.json`.
///
/// `port` is `None` when the config file is absent or unparseable — the daemon
/// badge shows "not configured" and all IPC commands return an error.
///
/// `DaemonClient` is stateless (just a port number); commands create one per
/// call rather than sharing a single instance.
pub struct AppState {
    /// TCP port the ringdrop daemon is listening on, if configured.
    pub port: Option<u16>,
}

impl AppState {
    /// Creates a new `AppState` from the given port.
    pub fn new(port: Option<u16>) -> Self {
        Self { port }
    }

    /// Returns a fresh `DaemonClient` for the configured port, or `None` if
    /// no port is configured.
    pub fn client(&self) -> Option<DaemonClient> {
        self.port.map(DaemonClient::new)
    }

    /// Sends `op` to the daemon and collects all [`EventKind::Record`] payloads
    /// into `Vec<T>`, deserialising each with serde.
    ///
    /// Non-`Record` events (Line, Progress, Done) are silently skipped — they
    /// are intended for the CLI's text renderer.
    ///
    /// # Errors
    ///
    /// Returns an error if the daemon is not configured, the connection fails,
    /// or the op itself returns an [`EventKind::Error`].
    pub(crate) async fn query<T: DeserializeOwned>(&self, op: Op) -> Result<Vec<T>, String> {
        let client = self.client().ok_or("daemon not configured")?;
        let mut kinds: Vec<EventKind> = Vec::new();
        client
            .send(op, |event| kinds.push(event.kind))
            .await
            .map_err(|e| e.to_string())?;
        Ok(collect_records(kinds))
    }

    /// Sends a fire-and-forget `op` to the daemon and returns `()` on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the daemon is not configured, the connection fails,
    /// or the op itself returns an [`EventKind::Error`].
    pub(crate) async fn execute(&self, op: Op) -> Result<(), String> {
        let client = self.client().ok_or("daemon not configured")?;
        client.send(op, |_| {}).await.map_err(|e| e.to_string())
    }
}

/// Collects [`EventKind::Record`] payloads from a sequence of event kinds,
/// deserialising each into `T`. Non-`Record` events are silently skipped.
///
/// Extracted as a free function so it can be unit-tested independently of the
/// daemon and reused across the command layer and its tests.
pub(crate) fn collect_records<T: DeserializeOwned>(events: Vec<EventKind>) -> Vec<T> {
    events
        .into_iter()
        .filter_map(|kind| {
            if let EventKind::Record { value } = kind {
                serde_json::from_value(value).ok()
            } else {
                None
            }
        })
        .collect()
}
