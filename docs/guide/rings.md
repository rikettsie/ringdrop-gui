# Rings & access control

Rings are the core of ringdrop's permission model. A ring is a named group
of peers. A blob is only served to a peer that belongs to a ring with
**Read** permission on that blob.

## How it works

```
blob ──tagged-with──► ring ──has-member──► peer
                        │
                        └── Permission::Read
```

1. You create a ring (e.g. `friends`).
2. You add peers to the ring.
3. You tag a blob with the ring.
4. Peers in the ring can download the blob.

Removing a peer from the ring immediately revokes their access — no keys
to rotate, no re-encryption.

## The open ring

The built-in **open** ring is publicly accessible — any peer can download
blobs tagged with it. Use it for truly public files.

It appears with a `pub` badge in the ring list and cannot have members
added manually.

## Creating a ring

Click **+** next to the Rings heading, type a name, and press Enter or ✓.

Ring names are arbitrary strings: `friends`, `work-team`, `family`.

## Adding peers

1. Select a ring from the list.
2. Click **Add peer**.
3. Paste the peer's base32 peer ID (they can copy it from their **ID** panel).
4. Press Enter or click **Add**.

If the peer is not yet in your address book they are registered automatically.

## Removing peers

Click the remove icon next to a member and confirm. The peer loses access
to all blobs tagged with that ring immediately.
