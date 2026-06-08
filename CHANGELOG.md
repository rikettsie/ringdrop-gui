# ringdrop-gui Changelog

All notable changes to this project will be documented in this file.

## [0.3.2] - 2026-06-08

### Bug Fixes

- (**license**) Fill bundle metadata fields in tauri.conf.json

## [0.3.1] - 2026-06-08

### Bug Fixes

- (**remote**) Show empty-state only after a completed fetch
- (**remote**) Propagate daemon Error events and fix download UX
- (**icons**) Generate tauri iconsvia presets

### Features

- Warn on ringdrop CLI/crate version mismatch at startup
- (**remote**) Allow concurrent downloads with per-blob progress bars

## [0.3.0] - 2026-06-08

### Bug Fixes

- (**docs**) Ensure the mermaid schema inside its section
- Use ringdrop-gui as product name for generated bundles

## [0.2.0] - 2026-06-08

### Bug Fixes

- Correctly show item hashes in panel lists
- Adjust many view details

### Documentation

- Add mascot on tauri v2

## [0.1.12] - 2026-06-02

### Bug Fixes

- Run svelte-kit sync before test and docs:build in CI
- Grant contents:write permission to release workflow
- Remove hardcoded version from tauri.conf.json, use Cargo.toml
- Remove stale directive and apply role to button

### Documentation

- Add README with architecture diagram and prerequisites
- Scaffold VitePress site with GitHub Pages deployment
- Add docs section to README, fix Node version to 24
- Use standard blue color for MIT license badge
- Rename Release badge to Installers
- Replace ASCII art diagram with Mermaid
- Fix typo in ringdrop Github repo

### Features

- Wire DaemonClient into app state via ringdrop 0.11.1
- Add DaemonBadge with Tailwind and polling daemon_status command
- Add blob_list, blob_import, blob_remove commands and BlobTable UI
- Add receive command with live progress bar via transfer_progress event
- Add IdPanel with peer-id display, copy, and amber QR code
- Add ring management panel with list, create, members, add/remove peer
- Add peer address book panel with add and remove
- Add grant management panel with grant and revoke
- Add remote catalog browser and grant/peer/remote panels — M1 complete
- Add changelog generation via git-cliff

### Refactoring

- Deduplicate types, extract query/execute helpers, add CI


