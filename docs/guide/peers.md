# Peers

The **Peers** panel is your local address book. It maps peer IDs to
human-readable nicknames and is shared with the rings system.

## Adding a peer

Click **Add peer** and fill in:

- **Peer ID** — the base32-encoded public key. Ask the peer to share it
  from their **ID** panel (or scan their QR code on mobile).
- **Nickname** — optional label you choose, e.g. `alice` or `work laptop`.
  Only visible to you locally.

If you add a peer to a ring before registering them here, they are
registered automatically with no nickname.

## Updating a nickname

Removing and re-adding a peer with the same ID updates the nickname.

## Removing a peer

Click the trash icon and confirm. The peer is removed from **all rings** and
from the address book. Their access to your blobs is revoked immediately.

::: tip
Removing a peer here does not notify them — they simply can no longer
download blobs from rings they were a member of.
:::
