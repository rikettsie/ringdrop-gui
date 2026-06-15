/** One row in the local blob table. */
export interface BlobRow {
  hash: string;
  name: string;
  rings: string[];
  ticket: string;
  /** e.g. `"file"` or `"dir, 3 files"` — absent for blobs from older daemons. */
  kind?: string;
  /** Number of files in a directory blob, if available. */
  file_count?: number | null;
  /** Total size in bytes, if available. */
  size_bytes?: number | null;
}

/** Result returned after a successful import. */
export interface ImportResult {
  hash: string;
  /** `"raw"` for files, `"hash_seq"` for directories. */
  format: string;
  name: string;
  ticket: string;
}

/** One ring in the local registry. */
export interface RingRow {
  name: string;
  /** `true` for the built-in open ring (publicly accessible to everyone). */
  open: boolean;
}

/** A peer — used both as a ring member and as an address-book entry. */
export interface PeerEntry {
  peer_id: string;
  nickname: string | null;
}

/** One catalog-access grant. */
export interface GrantRow {
  privilege: string;
  peer_id: string;
}

/** One blob entry from a remote peer's catalog. */
export interface RemoteBlobRow {
  hash: string;
  name: string;
  ticket: string;
  /** e.g. `"file"` or `"dir, 3 files"` — absent for blobs from older daemons. */
  kind?: string;
  /** Number of files in a directory blob, if available. */
  file_count?: number | null;
  /** Total size in bytes, if available. */
  size_bytes?: number | null;
}
