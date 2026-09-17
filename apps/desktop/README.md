# Scoop desktop app

Tauri 2 + React + Rust implementation of Scoop.

## Develop

```bash
npm install
npm run tauri dev
```

## Test

```bash
cd src-tauri
cargo test math_engine --lib
```

## Windows

| Label | Role |
|-------|------|
| `main` | Home, Library, Notes, History, Settings |
| `overlay` | Fullscreen marquee selection |
| `toolbar` | Contextual actions after capture |

## Modules (`src-tauri/src`)

`capture`, `ocr`, `classify`, `intent`, `math_engine`, `search`, `clipboard_ext`, `db`, `providers`, `commands`
