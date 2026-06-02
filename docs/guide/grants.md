# Grants

Grants control who can **query your blob catalog** — i.e. call `remote list`
on your node to see what you're sharing.

This is separate from ring membership. A peer needs:

1. The `blob-list` **grant** to see your catalog at all.
2. **Ring membership** with Read permission to see individual blobs in the
   catalog and download them.

## Why two layers?

Ring membership is about *what* a peer can download. Grants are about *who*
can even browse your catalog. You might want to let a peer see your catalog
without giving them access to any blobs yet — or keep your catalog private
entirely.

## Granting access

Click **Grant access**, paste the peer's ID, select `blob-list` from the
dropdown, and click **Grant**.

## Revoking access

Click the revoke icon next to a grant and confirm. The peer can no longer
query your catalog.

## Current privileges

`blob-list` is the only grantable privilege in this release. More privileges
may be added in future versions.
