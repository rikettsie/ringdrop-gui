use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Subset of ~/.ringdrop/config.json we care about.
/// Unknown fields (secret_key, relay_url, …) are silently ignored.
#[derive(Deserialize)]
struct RingdropConfig {
    daemon_port: u16,
}

/// What the frontend receives from `get_config`.
#[derive(Serialize)]
pub struct ConfigInfo {
    pub daemon_port: u16,
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ringdrop")
        .join("config.json")
}

fn load_config() -> Result<RingdropConfig, String> {
    let path = config_path();
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("invalid config: {e}"))
}

/// Returns the daemon port from ~/.ringdrop/config.json.
///
/// The frontend calls this once at startup to know which port the daemon
/// is listening on, then uses it for all subsequent DaemonClient calls.
#[tauri::command]
pub fn get_config() -> Result<ConfigInfo, String> {
    let cfg = load_config()?;
    Ok(ConfigInfo {
        daemon_port: cfg.daemon_port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_deserializes_daemon_port() {
        let json = r#"{"daemon_port": 7070, "secret_key": "somekey"}"#;
        let cfg: RingdropConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.daemon_port, 7070);
    }

    #[test]
    fn config_ignores_unknown_fields() {
        let json = r#"{"daemon_port": 9090, "relay_url": null, "extra": true}"#;
        let cfg: RingdropConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.daemon_port, 9090);
    }

    #[test]
    fn config_rejects_missing_port() {
        let result: Result<RingdropConfig, _> = serde_json::from_str(r#"{"other": 1}"#);
        assert!(result.is_err());
    }
}
