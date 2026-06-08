//! Tauri commands — thin wrappers around [`AppState::query`] and [`AppState::execute`].
//!
//! Every list command calls `state.query(Op::…)` and returns a typed `Vec<T>`.
//! Every mutation calls `state.execute(Op::…)` and returns `()`.
//! The [`receive`] command is the exception: it streams [`EventKind::Progress`]
//! events to the frontend via [`tauri::AppHandle::emit`].

use std::path::PathBuf;

use ringdrop::daemon::protocol::{EventKind, Op};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::state::AppState;

// ── Response types ────────────────────────────────────────────────────────────

/// Daemon connection status returned to the `DaemonBadge` component.
#[derive(Serialize)]
pub struct DaemonStatus {
    /// Whether the daemon is currently reachable.
    pub running: bool,
    /// Port the daemon is (or should be) listening on, if configured.
    pub port: Option<u16>,
}

/// One row in the local blob table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRow {
    /// BLAKE3 hex hash.
    pub hash: String,
    /// Tag name (usually the original filename).
    pub name: String,
    /// Ring names this blob is associated with.
    pub rings: Vec<String>,
    /// `rdrop://…` share ticket.
    pub ticket: String,
}

/// Result returned after a successful import.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    /// BLAKE3 hex hash of the imported blob.
    pub hash: String,
    /// `"raw"` for files, `"hash_seq"` for directories.
    pub format: String,
    /// Tag name (the original filename or directory name).
    pub name: String,
    /// `rdrop://…` share ticket.
    pub ticket: String,
}

/// One ring in the local registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingRow {
    /// Ring name (e.g. `"friends"`, `"work"`, or the built-in `"open"` ring).
    pub name: String,
    /// Whether this is the built-in open ring (publicly accessible to everyone).
    pub open: bool,
}

/// A peer — used both as a ring member and as an address-book entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerEntry {
    /// Base32 peer-id.
    pub peer_id: String,
    /// Human-readable label, if one was set.
    pub nickname: Option<String>,
}

/// One catalog-access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRow {
    /// Privilege name (e.g. `"blob-list"`).
    pub privilege: String,
    /// Base32 peer-id that holds this privilege.
    pub peer_id: String,
}

/// One blob entry from a remote peer's catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBlobRow {
    /// BLAKE3 hex hash.
    pub hash: String,
    /// File or collection name.
    pub name: String,
    /// `rdrop://…` share ticket.
    pub ticket: String,
}

/// GUI and daemon version numbers, baked in at compile time.
#[derive(Serialize)]
pub struct AppVersions {
    /// Version of this GUI application (from its `Cargo.toml`).
    pub gui: &'static str,
    /// Version of the `ringdrop` crate this binary was compiled against.
    pub daemon: &'static str,
}

// ── Commands ──────────────────────────────────────────────────────────────────

/// Returns the GUI and ringdrop crate versions, both resolved at compile time.
#[tauri::command]
pub fn app_versions() -> AppVersions {
    AppVersions {
        gui: env!("CARGO_PKG_VERSION"),
        daemon: env!("RINGDROP_VERSION"),
    }
}

/// Runs `rdrop --version` and returns the parsed version string.
///
/// # Errors
/// Returns an error if `rdrop` is not found in `PATH` or its output cannot be
/// parsed. The caller should treat this as a soft failure (warn, don't block).
#[tauri::command]
pub fn rdrop_cli_version() -> Result<String, String> {
    let output = std::process::Command::new("rdrop")
        .arg("--version")
        .output()
        .map_err(|e| format!("rdrop not found in PATH: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    // clap default format: "rdrop X.Y.Z"
    stdout
        .split_whitespace()
        .nth(1)
        .map(|s| s.trim().to_string())
        .ok_or_else(|| format!("unexpected rdrop --version output: {stdout}"))
}

/// Returns whether the ringdrop daemon is reachable and the configured port.
///
/// Called by `DaemonBadge` on a polling interval. Safe to call when no config
/// exists — returns `running: false`.
#[tauri::command]
pub async fn daemon_status(state: State<'_, AppState>) -> Result<DaemonStatus, String> {
    let running = match state.client() {
        None => false,
        Some(c) => c.is_running().await,
    };
    Ok(DaemonStatus {
        running,
        port: state.port,
    })
}

/// Lists all blobs in the local store.
#[tauri::command]
pub async fn blob_list(state: State<'_, AppState>) -> Result<Vec<BlobRow>, String> {
    state
        .query(Op::BlobList {
            peer: None,
            rings: None,
        })
        .await
}

/// Imports a file or directory into the local store.
///
/// `path` must be an absolute filesystem path. `rings` names the rings the
/// blob should be tagged with immediately after import; pass an empty slice to
/// skip tagging.
///
/// # Errors
///
/// Returns an error if the daemon is not configured, the path is unreadable,
/// or the import operation fails.
#[tauri::command]
pub async fn blob_import(
    state: State<'_, AppState>,
    path: String,
    rings: Vec<String>,
) -> Result<ImportResult, String> {
    let results: Vec<ImportResult> = state
        .query(Op::Import {
            path: PathBuf::from(path),
            rings,
            open: false,
        })
        .await?;
    results
        .into_iter()
        .next()
        .ok_or_else(|| "import succeeded but daemon returned no record".into())
}

/// Removes a blob from the local store.
///
/// `target` is a BLAKE3 hex hash or a file path previously imported. Ring
/// associations are removed automatically; disk space is reclaimed on the next
/// GC cycle (≈ 30 s).
#[tauri::command]
pub async fn blob_remove(state: State<'_, AppState>, target: String) -> Result<(), String> {
    state.execute(Op::BlobRemove { target }).await
}

/// Attaches a blob to the given rings (or the open ring).
///
/// `target` is a BLAKE3 hex hash or a file path previously imported.
/// Set `open` to `true` to make the blob publicly accessible, overriding `rings`.
///
/// # Errors
///
/// Returns an error if the daemon is not configured or the operation fails.
#[tauri::command]
pub async fn blob_attach(
    state: State<'_, AppState>,
    target: String,
    rings: Vec<String>,
    open: bool,
) -> Result<(), String> {
    state
        .execute(Op::BlobAttach {
            target,
            rings,
            open,
        })
        .await
}

/// Removes ring associations from a blob.
///
/// `target` is a BLAKE3 hex hash or a file path previously imported.
/// Set `open` to remove the open-ring association only.
/// Set `all` to strip every ring association.
///
/// # Errors
///
/// Returns an error if the daemon is not configured or the operation fails.
#[tauri::command]
pub async fn blob_detach(
    state: State<'_, AppState>,
    target: String,
    rings: Vec<String>,
    open: bool,
    all: bool,
) -> Result<(), String> {
    state
        .execute(Op::BlobDetach {
            target,
            rings,
            open,
            all,
        })
        .await
}

/// Lists all rings in the local registry.
#[tauri::command]
pub async fn ring_list(state: State<'_, AppState>) -> Result<Vec<RingRow>, String> {
    state.query(Op::RingList).await
}

/// Creates a new ring with the given name.
#[tauri::command]
pub async fn ring_create(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state.execute(Op::RingNew { name }).await
}

/// Lists all members of `ring`.
#[tauri::command]
pub async fn ring_members(
    state: State<'_, AppState>,
    ring: String,
) -> Result<Vec<PeerEntry>, String> {
    state.query(Op::RingMembers { ring }).await
}

/// Adds `peer` to `ring`. Registers the peer in the address book if absent.
#[tauri::command]
pub async fn ring_add(
    state: State<'_, AppState>,
    ring: String,
    peer: String,
) -> Result<(), String> {
    state.execute(Op::RingAdd { ring, peer }).await
}

/// Removes `peer` from `ring`.
#[tauri::command]
pub async fn ring_remove(
    state: State<'_, AppState>,
    ring: String,
    peer: String,
) -> Result<(), String> {
    state.execute(Op::RingRemove { ring, peer }).await
}

/// Lists all peers in the local address book.
#[tauri::command]
pub async fn peer_list(state: State<'_, AppState>) -> Result<Vec<PeerEntry>, String> {
    state.query(Op::PeerList).await
}

/// Adds a peer to the local address book, optionally with a nickname.
///
/// Idempotent: if the peer already exists the nickname is updated.
#[tauri::command]
pub async fn peer_add(
    state: State<'_, AppState>,
    peer: String,
    nickname: Option<String>,
) -> Result<(), String> {
    state.execute(Op::PeerAdd { peer, nickname }).await
}

/// Removes a peer from the address book and from all rings.
#[tauri::command]
pub async fn peer_remove(state: State<'_, AppState>, peer: String) -> Result<(), String> {
    state.execute(Op::PeerRemove { peer }).await
}

/// Lists current grants, optionally filtered by peer or privilege.
#[tauri::command]
pub async fn grant_list(
    state: State<'_, AppState>,
    peer: Option<String>,
    privilege: Option<String>,
) -> Result<Vec<GrantRow>, String> {
    state.query(Op::Grants { peer, privilege }).await
}

/// Grants `privilege` to `peer`.
#[tauri::command]
pub async fn grant_grant(
    state: State<'_, AppState>,
    peer: String,
    privilege: String,
) -> Result<(), String> {
    state.execute(Op::Grant { peer, privilege }).await
}

/// Revokes `privilege` from `peer`.
#[tauri::command]
pub async fn grant_revoke(
    state: State<'_, AppState>,
    peer: String,
    privilege: String,
) -> Result<(), String> {
    state.execute(Op::Revoke { peer, privilege }).await
}

/// Fetches the blob catalog from a remote peer.
///
/// The remote node must have granted `blob-list` to this node's identity.
#[tauri::command]
pub async fn remote_blob_list(
    state: State<'_, AppState>,
    peer: String,
) -> Result<Vec<RemoteBlobRow>, String> {
    state.query(Op::RemoteBlobList { peer }).await
}

/// Returns this node's peer-id as a base32 string.
///
/// The peer-id is the node's public key — share it so others can add this node
/// to their rings or grant it catalog access.
#[tauri::command]
pub async fn node_id(state: State<'_, AppState>) -> Result<String, String> {
    #[derive(Deserialize)]
    struct Rec {
        peer_id: String,
    }
    let recs: Vec<Rec> = state.query(Op::NodeId).await?;
    recs.into_iter()
        .next()
        .map(|r| r.peer_id)
        .ok_or_else(|| "daemon returned no peer_id record".into())
}

/// Downloads the blob described by `ticket` into `dest`.
///
/// Progress is streamed to the frontend as `"transfer_progress"` events with
/// payload `{ done: u64, total: u64 }`.
///
/// # Errors
///
/// Returns an error if the daemon is not configured, the ticket is invalid,
/// or the remote peer denies access.
#[tauri::command]
pub async fn receive(
    state: State<'_, AppState>,
    app: AppHandle,
    ticket: String,
    dest: String,
) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(
            Op::Receive {
                ticket,
                dest: PathBuf::from(dest),
                force_overwrite: false,
            },
            move |event| {
                if let EventKind::Progress { done, total } = event.kind {
                    let _ = app.emit(
                        "transfer_progress",
                        serde_json::json!({ "done": done, "total": total }),
                    );
                }
            },
        )
        .await
        .map_err(|e| e.to_string())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use crate::state::collect_records;
    use serde_json::json;

    use super::*;

    fn rec(value: serde_json::Value) -> EventKind {
        EventKind::Record { value }
    }

    #[test]
    fn collect_records_deserializes_blob_rows() {
        let rows: Vec<BlobRow> = collect_records(vec![rec(json!({
            "hash": "abc", "name": "f.txt", "rings": ["x"], "ticket": "rdrop://t"
        }))]);
        assert_eq!(rows[0].rings, vec!["x"]);
    }

    #[test]
    fn collect_records_skips_non_record_events() {
        let rows: Vec<BlobRow> = collect_records(vec![
            EventKind::Line {
                text: "2 blobs:".into(),
            },
            rec(json!({"hash":"abc","name":"f.txt","rings":[],"ticket":"rdrop://x"})),
            EventKind::Done,
        ]);
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn collect_records_ignores_progress_events() {
        let rows: Vec<BlobRow> = collect_records(vec![EventKind::Progress {
            done: 512,
            total: 1024,
        }]);
        assert!(rows.is_empty());
    }

    #[test]
    fn collect_records_deserializes_import_result() {
        let results: Vec<ImportResult> = collect_records(vec![rec(json!({
            "hash": "aa", "format": "raw", "name": "img.png", "ticket": "rdrop://aa"
        }))]);
        assert_eq!(results[0].format, "raw");
    }

    #[test]
    fn collect_records_deserializes_peer_entry() {
        let entries: Vec<PeerEntry> = collect_records(vec![
            rec(json!({"peer_id": "abc", "nickname": "alice"})),
            rec(json!({"peer_id": "def", "nickname": null})),
        ]);
        assert_eq!(entries[0].nickname, Some("alice".into()));
        assert_eq!(entries[1].nickname, None);
    }

    #[test]
    fn collect_records_deserializes_ring_rows() {
        let rows: Vec<RingRow> = collect_records(vec![
            rec(json!({"name": "friends", "open": false})),
            rec(json!({"name": "open", "open": true})),
        ]);
        assert!(rows[1].open);
    }

    #[test]
    fn collect_records_deserializes_grant_rows() {
        let rows: Vec<GrantRow> = collect_records(vec![rec(
            json!({"privilege": "blob-list", "peer_id": "abc"}),
        )]);
        assert_eq!(rows[0].privilege, "blob-list");
    }

    #[test]
    fn collect_records_deserializes_remote_blob_rows() {
        let rows: Vec<RemoteBlobRow> = collect_records(vec![rec(json!({
            "hash": "abc", "name": "video.mp4", "ticket": "rdrop://abc"
        }))]);
        assert_eq!(rows[0].name, "video.mp4");
    }

    #[test]
    fn collect_records_returns_empty_for_no_records() {
        let rows: Vec<BlobRow> = collect_records(vec![
            EventKind::Line {
                text: "No blobs.".into(),
            },
            EventKind::Done,
        ]);
        assert!(rows.is_empty());
    }

    #[test]
    fn node_id_record_is_deserializable() {
        #[derive(Deserialize)]
        struct Rec {
            peer_id: String,
        }
        let recs: Vec<Rec> = collect_records(vec![rec(json!({"peer_id": "abc32peerid"}))]);
        assert_eq!(recs[0].peer_id, "abc32peerid");
    }
}
