# ringdrop-gui

Desktop GUI for [ringdrop](https://github.com/RingDropSpace/ringdrop) — P2P file
sharing with ring-based access control.

Connects to a locally running ringdrop daemon over IPC and exposes the full
`rdrop` CLI surface as a native UI: browse and share blobs, manage rings and
peers, grant catalog access, and download from tickets with live progress.

## Architecture

```
┌──────────────────────────────┐
│    ringdrop-gui (Tauri v2)   │
│  ┌───────────┐ ┌───────────┐ │
│  │  Svelte   │ │   Tauri   │ │
│  │ frontend  │◄►  backend  │ │
│  └───────────┘ └─────┬─────┘ │
└───────────────────── │ ──────┘
                       │ TCP localhost
                       │ JSON (Op / EventKind)
                       ▼
┌──────────────────────────────┐
│      ringdrop daemon         │
│  ┌────────────────────────┐  │
│  │        Node            │  │
│  │  FsStore  Registry     │  │
│  │  Grants   Peers        │  │
│  └────────────────────────┘  │
└──────────────────────────────┘
```

The Tauri backend reads `~/.ringdrop/config.json` to locate the daemon port,
then forwards every UI action as a `DaemonClient` IPC call. No separate
configuration; no embedded node.

## Prerequisites

- **ringdrop daemon** — install the CLI and start the daemon:
  ```sh
  cargo install ringdrop
  rdrop daemon start
  ```
- **Node ≥ 24** — `node --version` (use [fnm](https://github.com/Schniz/fnm) to match `.node-version`)
- **Rust stable** — `rustup update stable`
- **Linux system deps** (Tauri webview):
  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev
  ```
  See [Tauri prerequisites](https://tauri.app/start/prerequisites/) for macOS
  and Windows.

## Development

```sh
npm install
npm run tauri dev
```

## Build

```sh
npm run tauri build
# → src-tauri/target/release/bundle/
#   Linux:   .AppImage  .deb  .rpm
#   macOS:   .dmg
#   Windows: .msi
```

Pre-built installers for all platforms are available on the
[Releases](../../releases) page.

## Docs

User-facing documentation lives in [`docs/`](./docs) and is published to
GitHub Pages on every push to `main`.

```sh
npm run docs:dev      # live-reload at localhost:5173
npm run docs:build    # production build → docs/.vitepress/dist
```
