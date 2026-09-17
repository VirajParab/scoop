# Recommended technical stack

**Status:** Draft — engineering may adjust; architecture constraints stay fixed.  
**Related ADRs:** [ADR-001](../decisions/ADR-001-desktop-framework.md), [ADR-002](../decisions/ADR-002-ocr.md), [ADR-003](../decisions/ADR-003-database.md)

## Goals for stack choice

1. **Lightweight** — Scoop is a utility, not an Electron-heavy suite by default
2. **Strong Linux desktop integration** — hotkeys, overlays, capture, Wayland/X11
3. **Fast cold invoke** — tray-resident process
4. **Safe local persistence** — SQLite
5. **Pluggable AI/search** — clear interfaces

## Recommendation (MVP)

| Layer | Choice | Rationale |
|-------|--------|-----------|
| App shell | **Tauri 2** (Rust + small web UI) | Small binary vs Electron; Rust for capture/OCR/FFI; web UI for toolbar/settings speed |
| Core logic | **Rust** | Capture, OCR bindings, math, DB, providers |
| UI surfaces | **HTML/CSS/TS** (Tauri webviews) for panels; native overlay if needed for selection | Fast iteration on toolbar/settings |
| DB | **SQLite** + FTS5 | Notes/history/settings |
| OCR | **Tesseract** (primary) or **PaddleOCR** eval | Local, offline; EN first |
| Math | Local expression parser (e.g. `meval` / `fasteval` class) + currency/% helpers | Deterministic |
| AI HTTP | `reqwest` + provider adapters | OpenAI-compatible + Anthropic |
| Clipboard | Arboard / wl-clipboard / xclip abstractions | Via desktop backend |
| Packaging | AppImage + `.deb` first | Tester distribution |

### Alternatives considered

| Option | When to prefer |
|--------|----------------|
| **Electron** | Team velocity on JS-only; accept larger footprint |
| **GTK / Qt native** | Maximum native feel; slower UI iteration for rich result panels |
| **Pure Rust GUI (egui/iced)** | Minimal deps; weaker “document-like” notes UI |

**Decision bias:** Tauri unless Linux capture/hotkey blockers force a native GTK/Qt overlay companion.

## Component map

```text
tauri-shell/
  ui/           # settings, notes, toolbar, ask-ai panel
src-tauri/
  hotkey/
  capture/
  ocr/
  classify/
  actions/      # search, math, ask, notes
  providers/
  db/
  privacy/
```

## External services

| Service | Role |
|---------|------|
| AI provider APIs | Vision + text |
| System browser | Open search URLs |
| Secret Service | API keys |

## Versioning

- SemVer for app releases
- DB migrations numbered independently
- Provider adapters versioned with capability flags

## Open decisions to close before coding week 1

1. Confirm Tauri 2 Wayland global shortcut story on GNOME/KDE
2. OCR: Tesseract vs PaddleOCR bake-off on code + dark themes
3. Selection overlay: webview vs native layer shell window
