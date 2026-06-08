//! Reads the daemon port from ~/.ringdrop/config.json.
//!
//! Delegates parsing to [`ringdrop::config::Config`] so the GUI inherits the
//! same field defaults (including the default port) without duplicating them.
//! The file is never created or modified by the GUI.

use std::path::PathBuf;

use ringdrop::config::Config;

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ringdrop")
        .join("config.json")
}

/// Returns the daemon port from `~/.ringdrop/config.json`, or `None` if the
/// file is absent or cannot be parsed.
///
/// When the file exists but omits `daemon_port`, the field's serde default
/// from [`ringdrop::config::Config`] applies (currently 60001).
pub(crate) fn read_daemon_port() -> Option<u16> {
    let raw = std::fs::read_to_string(config_path()).ok()?;
    serde_json::from_str::<Config>(&raw)
        .ok()
        .map(|c| c.daemon_port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_explicit_port() {
        // secret_key must be a valid 32-byte array for ringdrop's Config to parse.
        let key = serde_json::Value::Array(vec![serde_json::json!(0); 32]);
        let json = serde_json::json!({ "daemon_port": 7070, "secret_key": key }).to_string();
        let cfg: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.daemon_port, 7070);
    }

    #[test]
    fn uses_default_port_when_field_absent() {
        let key = serde_json::Value::Array(vec![serde_json::json!(0); 32]);
        let json = serde_json::json!({ "secret_key": key }).to_string();
        let cfg: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.daemon_port, 60001);
    }

    #[test]
    fn read_daemon_port_returns_none_on_bad_json() {
        assert!(serde_json::from_str::<Config>("not json").is_err());
    }
}
