/** One row in the local blob table. */
export interface BlobRow {
  hash: string;
  name: string;
  rings: string[];
  ticket: string;
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
}
