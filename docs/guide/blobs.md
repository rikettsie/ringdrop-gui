# Blobs

The **Blobs** panel is the main view. It lists every file or directory you
have imported into the local store.

## Columns

| Column | Description |
|---|---|
| Name | Tag name — usually the original filename |
| Hash | First 12 characters of the BLAKE3 hex hash |
| Rings | Ring badges. **untagged** means no one can download it yet |

## Import

Click **Import** to open a file picker. The daemon imports the file,
computes its BLAKE3 hash, writes it to the blob store, and generates a
share ticket.

Imported blobs are untagged by default. Tag them with a ring via the
**Rings** panel to make them downloadable.

## Copy ticket

Click the copy icon on any row to copy the `rdrop://…` share ticket to the
clipboard. Send this ticket to a peer so they can download the file directly
with `rdrop receive <ticket>` or via the **Receive** panel.

## Delete

Click the trash icon and confirm. The daemon removes all ring tags and
deletes the blob. Disk space is reclaimed on the next GC cycle (≈ 30 s).

::: warning
Deletion is permanent. There is no undo. Make sure you have another copy
before removing a blob.
:::
