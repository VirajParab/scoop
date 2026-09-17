# Coding standards

**Status:** Draft

## Principles

1. **Speed of the core loop** beats feature cleverness
2. **Privacy defaults** are not optional flags buried in code paths
3. **Provider boundaries** stay clean — no vendor SDK types leaking into UI
4. Prefer small modules with explicit interfaces over god-objects

## Module boundaries

| May depend on | Must not depend on |
|---------------|--------------------|
| `actions` → `providers`, `db`, `ocr` | `providers` → UI |
| `ui` → commands/API façade | `ui` → raw HTTP to OpenAI |
| `capture` → desktop backend | `capture` → notes schema |

Public façade: command layer (Tauri commands / IPC) is the only UI↔core bridge.

## Rust guidelines (when using Rust core)

- `clippy` clean on CI (`-D warnings` when feasible)
- No `unwrap()` in production paths — use `Result` + user-facing error mapping
- Async: `tokio`; cancel AI requests when UI dismisses
- Temp files: RAII / guarded cleanup
- Secrets: never `Debug`-print API keys

## TypeScript / UI guidelines

- Keep toolbar and result panels lightweight — avoid heavy design systems
- Accessible: Escape cancels; focus traps in modals; contrast on overlays
- No silent catch — surface toast/inline errors
- Don’t store API keys in `localStorage`; call core secret APIs

## Error UX

Map errors to stable codes:

```text
HOTKEY_CONFLICT
CAPTURE_FAILED
OCR_FAILED
PROVIDER_UNCONFIGURED
PROVIDER_RATE_LIMIT
NETWORK_ERROR
DB_ERROR
```

User message + optional “Open Settings” CTA.

## Commits & PRs

- Imperative subject focused on **why**
- Link feature spec section when implementing P0 behavior
- Include test plan for capture/hotkey changes (display server noted)

## Documentation

- Update feature spec **Status** / behavior notes when shipping deviations
- New architectural choice → ADR in `docs/decisions/`

## What not to do

- Permanently write captures without an explicit save path
- Block toolbar rendering on slow vision classify
- Call cloud AI for pure arithmetic that the local engine can handle
- Add Electron-scale dependencies for minor UI polish
