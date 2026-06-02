//! Shared application state injected into every Tauri command.

use ringdrop::daemon::client::DaemonClient;

/// Holds the daemon port read at startup from `~/.ringdrop/config.json`.
///
/// `port` is `None` when the config file is absent or unparseable — the daemon
/// badge will show "not configured" and all IPC commands will return an error.
///
/// `DaemonClient` is stateless (just a port number); commands create one
/// per call rather than sharing a single instance.
pub struct AppState {
    pub port: Option<u16>,
}

impl AppState {
    pub fn new(port: Option<u16>) -> Self {
        Self { port }
    }

    /// Returns a fresh `DaemonClient` for the configured port, or `None` if
    /// no port is configured.
    pub fn client(&self) -> Option<DaemonClient> {
        self.port.map(DaemonClient::new)
    }
}
