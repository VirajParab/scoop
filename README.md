# Scoop

**AI-powered visual workspace for Linux.**

Scoop lets you select anything on your screen and instantly search it, calculate it, understand it, turn it into a note, or save it to your local library.

> **Select → Understand → Act → Save**

## Status

MVP implementation in progress under [`apps/desktop`](./apps/desktop).

## Quick start

```bash
make install
make dev
```

Or manually:

```bash
cd apps/desktop
npm install
npm run tauri dev
```

**Default hotkey:** `Super+Shift+Space` (tray → **Select** if registration fails).

Optional: install `tesseract-ocr` + `tesseract-ocr-eng` for OCR.

## Docs

| Document | Description |
|----------|-------------|
| [Docs index](./docs/README.md) | Full documentation map |
| [Dev setup](./docs/engineering/development-setup.md) | Local development |
| [QA checklist](./docs/engineering/qa-checklist.md) | Manual release checks |
| [MVP scope](./docs/product/mvp-scope.md) | What ships in v1 |

## Core loop

1. **Global hotkey** — invoke from anywhere
2. **Marquee select** — drag a rectangle over screen content
3. **Understand** — OCR + classification heuristics
4. **Act** — Search, Calculate, Ask AI, Copy, Library, Note
5. **Save** — local library / notes / history (privacy-first)

## License

TBD
