//! Reads the daemon port from ~/.ringdrop/config.json.
//!
//! The GUI only needs the port — unknown fields (secret_key, relay_url, …)
//! are silently ignored. The file is never created or modified by the GUI.

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct RingdropConfig {
    daemon_port: u16,
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ringdrop")
        .join("config.json")
}

/// Returns the daemon port from `~/.ringdrop/config.json`, or `None` if the
/// file is absent or cannot be parsed.
pub(crate) fn read_daemon_port() -> Option<u16> {
    let raw = std::fs::read_to_string(config_path()).ok()?;
    serde_json::from_str::<RingdropConfig>(&raw)
        .ok()
        .map(|c| c.daemon_port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_daemon_port() {
        let json = r#"{"daemon_port": 7070, "secret_key": "somekey"}"#;
        let cfg: RingdropConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.daemon_port, 7070);
    }

    #[test]
    fn ignores_unknown_fields() {
        let json = r#"{"daemon_port": 9090, "relay_url": null, "extra": true}"#;
        let cfg: RingdropConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.daemon_port, 9090);
    }

    #[test]
    fn rejects_missing_port() {
        let result: Result<RingdropConfig, _> = serde_json::from_str(r#"{"other": 1}"#);
        assert!(result.is_err());
    }

    #[test]
    fn read_daemon_port_returns_none_on_bad_json() {
        // read_daemon_port uses Option chaining — bad JSON yields None, not a panic.
        // We can verify this by exercising the serde path directly.
        let result: Result<RingdropConfig, _> = serde_json::from_str("not json");
        assert!(result.is_err());
    }
}
