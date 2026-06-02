# Quickstart

## 1. Start the daemon

ringdrop-gui is a frontend for the ringdrop daemon. Start it first:

```sh
rdrop daemon start
```

The status badge in the top-right corner of the app will turn green
(**connected**) once the GUI detects the daemon.

## 2. Share your peer ID

Click **ID** in the sidebar. Your peer ID is a base32-encoded public key —
share it so others can add you to their rings.

Use the **Copy** button or let peers scan the amber QR code.

## 3. Import a file

Click **Blobs → Import** and pick a file or directory. The daemon imports it,
computes its BLAKE3 hash, and generates a share ticket (`rdrop://…`).

The blob is untagged at this point — no one can download it yet.

## 4. Create a ring and tag the blob

1. Go to **Rings → +** and create a ring (e.g. `friends`).
2. Add peers to the ring using their peer IDs.
3. Back in **Blobs**, the ring badge appears on the row. Use the tag editor
   (coming in the next release) to associate the blob with the ring.

Once tagged, peers in the ring can query your catalog and download the file.

## 5. Download from a peer

1. Ask a peer for their peer ID.
2. Go to **Remote**, paste their peer ID and click **Browse**.
3. Their catalog appears — click **Download** on any row, pick a destination,
   and watch the progress bar.

::: tip
Make sure the peer has granted you **blob-list** access (via **Grants**) and
added you to the ring that contains the blob.
:::
