# Install

## Pre-built installers

Download the latest installer for your platform from the
[Releases](https://github.com/rikettsie/ringdrop-gui/releases) page.

| Platform | File |
|---|---|
| Linux | `.AppImage` (portable) or `.deb` / `.rpm` |
| macOS | `.dmg` |
| Windows | `.msi` |

### Linux — AppImage

```sh
chmod +x ringdrop_*.AppImage
./ringdrop_*.AppImage
```

### macOS

Open the `.dmg`, drag **ringdrop** into Applications, then right-click →
Open on first launch (Gatekeeper warning).

### Windows

Run the `.msi` installer. Windows Defender SmartScreen may warn on first
launch — click **More info → Run anyway**.

---

## Prerequisites

ringdrop-gui connects to a **ringdrop daemon** running locally. The daemon
must be installed and started before launching the GUI.

```sh
# Install the CLI (includes the daemon)
cargo install ringdrop

# Start the daemon
rdrop daemon start
```

The GUI reads `~/.ringdrop/config.json` to find the daemon port. If the file
is absent the status badge will show **not configured**.

### Linux system libraries

Required by the Tauri webview:

```sh
# Debian / Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel libappindicator-gtk3-devel
```

---

## Build from source

```sh
# Prerequisites: Rust stable, Node ≥ 24, system deps above

git clone https://github.com/rikettsie/ringdrop-gui
cd ringdrop-gui
npm install
npm run tauri build
# → src-tauri/target/release/bundle/
```

## Develop

```sh
npm install
make hooks        # activate git hooks (once, after cloning)
npm run tauri dev
```

The SvelteKit frontend reloads on file changes. The Rust backend recompiles
automatically when `src-tauri/` files are modified. A running ringdrop daemon
is still required (see [Prerequisites](#prerequisites) above).
