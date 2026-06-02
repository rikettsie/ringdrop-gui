//! Tauri commands — thin wrappers around `DaemonClient::send`.
//!
//! Every command follows the same pattern:
//! ```text
//! state.client()  →  DaemonClient::send(Op::…, |event| collect EventKind::Record values)
//! ```

use std::path::PathBuf;

use ringdrop::daemon::protocol::{EventKind, Op};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::state::AppState;

// ── Response types ────────────────────────────────────────────────────────────

/// Daemon connection status returned to `DaemonBadge`.
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

// ── Helper ────────────────────────────────────────────────────────────────────

/// Collects `EventKind::Record` payloads from a sequence of event kinds,
/// deserialising each into `T`. Non-`Record` events are silently skipped.
///
/// Extracted so it can be tested independently of the daemon.
fn collect_records<T: serde::de::DeserializeOwned>(events: Vec<EventKind>) -> Vec<T> {
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

// ── Commands ──────────────────────────────────────────────────────────────────

/// Returns whether the ringdrop daemon is reachable and the configured port.
///
/// Called by `DaemonBadge` on a polling interval. Safe to call when no config
/// exists — returns `running: false`.
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

/// Lists all blobs in the local store.
///
/// Returns one [`BlobRow`] per blob, each carrying the hash, name, ring
/// associations, and share ticket.
#[tauri::command]
pub async fn blob_list(state: State<'_, AppState>) -> Result<Vec<BlobRow>, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::BlobList { peer: None, rings: None }, |event| {
            kinds.push(event.kind);
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(collect_records(kinds))
}

/// Imports a file or directory into the local store.
///
/// `path` must be an absolute filesystem path. `rings` names the rings the
/// blob should be tagged with immediately after import; pass an empty slice
/// to skip tagging (the blob will be untagged until the user edits its rings).
#[tauri::command]
pub async fn blob_import(
    state: State<'_, AppState>,
    path: String,
    rings: Vec<String>,
) -> Result<ImportResult, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(
            Op::Import {
                path: PathBuf::from(path),
                rings,
                open: false,
            },
            |event| kinds.push(event.kind),
        )
        .await
        .map_err(|e| e.to_string())?;
    collect_records::<ImportResult>(kinds)
        .into_iter()
        .next()
        .ok_or_else(|| "import succeeded but daemon returned no record".into())
}

/// Removes a blob from the local store.
///
/// `target` is a BLAKE3 hex hash or a file path previously imported.
/// Ring associations are removed automatically; disk space is reclaimed on
/// the next GC cycle (≈ 30 s).
#[tauri::command]
pub async fn blob_remove(state: State<'_, AppState>, target: String) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::BlobRemove { target }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// One ring in the local registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingRow {
    /// Ring name (e.g. `"friends"`, `"work"`, or the built-in `"open"` ring).
    pub name: String,
    /// Whether this is the built-in open ring (publicly accessible to everyone).
    pub open: bool,
}

/// One member of a ring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRow {
    /// Base32 peer-id.
    pub peer_id: String,
    /// Human-readable label, if one was set.
    pub nickname: Option<String>,
}

/// Lists all rings in the local registry.
#[tauri::command]
pub async fn ring_list(state: State<'_, AppState>) -> Result<Vec<RingRow>, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::RingList, |event| kinds.push(event.kind))
        .await
        .map_err(|e| e.to_string())?;
    Ok(collect_records(kinds))
}

/// Creates a new ring with the given name.
#[tauri::command]
pub async fn ring_create(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::RingNew { name }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// Lists all members of `ring`.
#[tauri::command]
pub async fn ring_members(
    state: State<'_, AppState>,
    ring: String,
) -> Result<Vec<MemberRow>, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::RingMembers { ring }, |event| kinds.push(event.kind))
        .await
        .map_err(|e| e.to_string())?;
    Ok(collect_records(kinds))
}

/// Adds `peer` to `ring`. Registers the peer in the address book if absent.
#[tauri::command]
pub async fn ring_add(
    state: State<'_, AppState>,
    ring: String,
    peer: String,
) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::RingAdd { ring, peer }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// Removes `peer` from `ring`.
#[tauri::command]
pub async fn ring_remove(
    state: State<'_, AppState>,
    ring: String,
    peer: String,
) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::RingRemove { ring, peer }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// One entry in the local peer address book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRow {
    /// Base32 peer-id.
    pub peer_id: String,
    /// Human-readable label, if one was set.
    pub nickname: Option<String>,
}

/// Lists all peers in the local address book.
#[tauri::command]
pub async fn peer_list(state: State<'_, AppState>) -> Result<Vec<PeerRow>, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::PeerList, |event| kinds.push(event.kind))
        .await
        .map_err(|e| e.to_string())?;
    Ok(collect_records(kinds))
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
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::PeerAdd { peer, nickname }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// Removes a peer from the address book and from all rings.
#[tauri::command]
pub async fn peer_remove(state: State<'_, AppState>, peer: String) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::PeerRemove { peer }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// One catalog-access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRow {
    /// Privilege name (e.g. `"blob-list"`).
    pub privilege: String,
    /// Base32 peer-id that holds this privilege.
    pub peer_id: String,
}

/// Lists current grants, optionally filtered by peer or privilege.
#[tauri::command]
pub async fn grant_list(
    state: State<'_, AppState>,
    peer: Option<String>,
    privilege: Option<String>,
) -> Result<Vec<GrantRow>, String> {
    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::Grants { peer, privilege }, |event| kinds.push(event.kind))
        .await
        .map_err(|e| e.to_string())?;
    Ok(collect_records(kinds))
}

/// Grants `privilege` to `peer`.
#[tauri::command]
pub async fn grant_grant(
    state: State<'_, AppState>,
    peer: String,
    privilege: String,
) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::Grant { peer, privilege }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// Revokes `privilege` from `peer`.
#[tauri::command]
pub async fn grant_revoke(
    state: State<'_, AppState>,
    peer: String,
    privilege: String,
) -> Result<(), String> {
    let client = state.client().ok_or("daemon not configured")?;
    client
        .send(Op::Revoke { peer, privilege }, |_| {})
        .await
        .map_err(|e| e.to_string())
}

/// Returns this node's peer-id as a base32 string.
///
/// The peer-id is the node's public key encoded in base32 — share it so
/// others can add this node to their rings or grant it catalog access.
#[tauri::command]
pub async fn node_id(state: State<'_, AppState>) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Rec {
        peer_id: String,
    }

    let client = state.client().ok_or("daemon not configured")?;
    let mut kinds: Vec<EventKind> = Vec::new();
    client
        .send(Op::NodeId, |event| kinds.push(event.kind))
        .await
        .map_err(|e| e.to_string())?;
    collect_records::<Rec>(kinds)
        .into_iter()
        .next()
        .map(|r| r.peer_id)
        .ok_or_else(|| "daemon returned no peer_id record".into())
}

/// Downloads the blob described by `ticket` into the directory `dest`.
///
/// Progress is streamed to the frontend as `"transfer_progress"` events with
/// payload `{ done: u64, total: u64 }`. The frontend should call
/// `listen("transfer_progress", …)` before invoking this command.
///
/// `dest` must be an absolute path to an existing directory.
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
    use super::*;
    use serde_json::json;

    fn record_kind(value: serde_json::Value) -> EventKind {
        EventKind::Record { value }
    }

    #[test]
    fn collect_records_deserializes_blob_rows() {
        let events = vec![
            record_kind(json!({
                "hash": "abc123",
                "name": "photo.jpg",
                "rings": ["friends"],
                "ticket": "rdrop://abc"
            })),
            record_kind(json!({
                "hash": "def456",
                "name": "doc.pdf",
                "rings": [],
                "ticket": "rdrop://def"
            })),
        ];
        let rows: Vec<BlobRow> = collect_records(events);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].hash, "abc123");
        assert_eq!(rows[0].rings, vec!["friends"]);
        assert_eq!(rows[1].rings, Vec::<String>::new());
    }

    #[test]
    fn collect_records_skips_non_record_events() {
        let events = vec![
            EventKind::Line { text: "2 blobs:".into() },
            record_kind(json!({"hash":"abc","name":"f.txt","rings":[],"ticket":"rdrop://x"})),
            EventKind::Done,
        ];
        let rows: Vec<BlobRow> = collect_records(events);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].hash, "abc");
    }

    #[test]
    fn collect_records_deserializes_import_result() {
        let events = vec![record_kind(json!({
            "hash": "aabbcc",
            "format": "raw",
            "name": "image.png",
            "ticket": "rdrop://aabbcc"
        }))];
        let results: Vec<ImportResult> = collect_records(events);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].format, "raw");
        assert_eq!(results[0].ticket, "rdrop://aabbcc");
    }

    #[test]
    fn collect_records_deserializes_grant_rows() {
        let events = vec![
            record_kind(json!({"privilege": "blob-list", "peer_id": "abc"})),
        ];
        let rows: Vec<GrantRow> = collect_records(events);
        assert_eq!(rows[0].privilege, "blob-list");
        assert_eq!(rows[0].peer_id, "abc");
    }

    #[test]
    fn collect_records_deserializes_peer_rows() {
        let events = vec![
            record_kind(json!({"peer_id": "abc", "nickname": "alice"})),
            record_kind(json!({"peer_id": "def", "nickname": null})),
        ];
        let rows: Vec<PeerRow> = collect_records(events);
        assert_eq!(rows[0].nickname, Some("alice".into()));
        assert_eq!(rows[1].peer_id, "def");
        assert_eq!(rows[1].nickname, None);
    }

    #[test]
    fn collect_records_deserializes_ring_rows() {
        let events = vec![
            record_kind(json!({"name": "friends", "open": false})),
            record_kind(json!({"name": "open",    "open": true})),
        ];
        let rows: Vec<RingRow> = collect_records(events);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "friends");
        assert!(!rows[0].open);
        assert!(rows[1].open);
    }

    #[test]
    fn collect_records_deserializes_member_rows() {
        let events = vec![
            record_kind(json!({"peer_id": "abc", "nickname": "alice"})),
            record_kind(json!({"peer_id": "def", "nickname": null})),
        ];
        let rows: Vec<MemberRow> = collect_records(events);
        assert_eq!(rows[0].nickname, Some("alice".into()));
        assert_eq!(rows[1].nickname, None);
    }

    #[test]
    fn node_id_record_is_deserializable() {
        #[derive(serde::Deserialize)]
        struct Rec { peer_id: String }
        let events = vec![record_kind(json!({ "peer_id": "abc32peerid" }))];
        let recs: Vec<Rec> = collect_records(events);
        assert_eq!(recs[0].peer_id, "abc32peerid");
    }

    #[test]
    fn collect_records_ignores_progress_events() {
        // Progress events are handled separately (streamed via AppHandle::emit);
        // they must never end up in the record collector.
        let events = vec![
            EventKind::Progress { done: 512, total: 1024 },
            EventKind::Progress { done: 1024, total: 1024 },
            EventKind::Done,
        ];
        let rows: Vec<BlobRow> = collect_records(events);
        assert!(rows.is_empty());
    }

    #[test]
    fn collect_records_returns_empty_for_no_records() {
        let events = vec![
            EventKind::Line { text: "No blobs in local store.".into() },
            EventKind::Done,
        ];
        let rows: Vec<BlobRow> = collect_records(events);
        assert!(rows.is_empty());
    }
}
