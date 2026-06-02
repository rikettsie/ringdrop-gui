# Remote catalog

The **Remote** panel lets you browse and download blobs from another node.

This is the payoff of ring-based access control: peers curate what they
share per ring, and you see only what you have permission to see.

## Requirements

Before you can browse a peer's catalog:

1. **The peer must have granted you `blob-list`** — they do this via their
   **Grants** panel.
2. **You must be a member of at least one of their rings** — they add you
   via their **Rings** panel.

Only blobs tagged with rings you belong to appear in the catalog.

## Browsing

1. Ask the peer for their peer ID.
2. Paste it in the **Remote peer ID** field and click **Browse**.
3. The catalog loads — each row shows the filename and a truncated hash.

## Downloading

Click **Download** on any row, pick a destination directory, and the
transfer starts. A live progress bar shows bytes transferred and total size.

The download uses the same `rdrop://…` ticket the remote peer generated
when they imported the file. If the peer is offline the download will fail —
ringdrop is direct P2P with relay fallback, not a CDN.

## Receive from ticket

If a peer shares a raw ticket (`rdrop://…`) directly — via chat, email, or
QR code — use the **Receive** panel instead. No catalog browsing needed;
paste the ticket and pick a destination.
