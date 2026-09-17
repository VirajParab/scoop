# Development setup

**Status:** Accepted  
**Audience:** Contributors setting up Scoop on Linux

## Prerequisites

| Tool | Notes |
|------|-------|
| Linux (Ubuntu 22.04+ / Fedora 39+) | Wayland or X11 |
| Rust stable (`rustup`) | Tauri core |
| Node.js 20+ and npm | UI |
| Tauri Linux deps | `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, build-essential |
| Tesseract (recommended) | `tesseract-ocr` + `tesseract-ocr-eng` for OCR |
| Optional clipboard helpers | `wl-clipboard` / `xclip` |

### Ubuntu packages (example)

```bash
sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev \
  patchelf libssl-dev tesseract-ocr tesseract-ocr-eng
```

## Clone & install

```bash
git clone <repo-url> scoop
cd scoop
make install
```

## Run

```bash
make dev
```

## Build

```bash
make build          # full Tauri app
make frontend       # UI only
```

Outputs AppImage / `.deb` under `apps/desktop/src-tauri/target/release/bundle/` when bundling succeeds.

## Test / check

```bash
make test           # math engine tests
make check          # cargo check
make typecheck      # TypeScript
make help           # list all targets
```

## Environment

| Variable | Purpose |
|----------|---------|
| `SCOOP_LOG` | Reserved for future log level |
| `OPENAI_API_KEY` | Optional; prefer Settings → API key (OS keyring) |

Never commit real API keys.

## Data locations

| Path | Contents |
|------|----------|
| `$XDG_DATA_HOME/scoop/scoop.db` | Library, notes, history, settings |
| `$XDG_DATA_HOME/scoop/media/` | Saved screenshots |
| `$XDG_CACHE_HOME/scoop/captures/` | Temporary selection captures |

Reset local data:

```bash
rm -rf "${XDG_DATA_HOME:-$HOME/.local/share}/scoop" "${XDG_CACHE_HOME:-$HOME/.cache}/scoop"
```

## Useful checks

```bash
cd apps/desktop/src-tauri
cargo test math_engine
cargo clippy

cd ../
npx tsc --noEmit
```

## Hotkey / capture debugging

1. `echo $XDG_SESSION_TYPE` (wayland vs x11)
2. Use tray menu **Select (Scoop)** if global hotkey fails to register
3. Default hotkey: `Super+Shift+Space` (fallback attempted: `Ctrl+Shift+Space`)

## Docs

- Architecture: [../architecture/overview.md](../architecture/overview.md)
- Feature specs: [../features/](../features/)
