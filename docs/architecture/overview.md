# System architecture overview

**Status:** Accepted (directional)  
**Related:** [desktop.md](./desktop.md) · [ai-providers.md](./ai-providers.md) · [data-model.md](./data-model.md)

## High-level flow

```text
              Linux Desktop
                   │
              Global Hotkey
                   │
            Marquee Selector
                   │
             Screen Capture
                   │
        ┌──────────┴──────────┐
        │                     │
       OCR                  Vision
        │                     │
        └──────────┬──────────┘
                   │
             Intent Engine
                   │
          ┌────────┼────────┐
          │        │        │
        Search   Math      AI
          │        │        │
          └────────┼────────┘
                   │
              Result Layer
                   │
        ┌──────────┼──────────┬──────────┐
        │          │          │          │
      Copy     Library     Notes     Preview*
```

\* Library = arranged screenshots & text (searchable anytime). Preview (HTML) is v2.

## Logical modules

| Module | Responsibility |
|--------|----------------|
| **Shell / Hotkey** | Global shortcut registration, tray, lifecycle |
| **Selection UI** | Overlay, marquee, dim, resize, cancel |
| **Capture** | Region screenshot, DPI/monitor math, temp files |
| **OCR** | Image → text; editable buffer |
| **Vision / Classify** | Content type labels for toolbar |
| **Intent Engine** | Rank/filter actions from type + heuristics |
| **Actions** | Search, Math, Ask AI, Library, Note, Copy, … |
| **Result UI** | Floating panels, preview windows |
| **Persistence** | SQLite: library, notes, history, settings |
| **Providers** | AI + search abstractions |
| **Privacy** | Retention, cleanup, future exclusions |

## Process model (recommended)

Prefer a **single long-running background process** (tray / daemon) plus **ephemeral UI surfaces**:

1. **Daemon** — hotkey, capture orchestration, DB, provider clients
2. **Overlay window** — selection mode (fullscreen transparent/compositor-aware)
3. **Toolbar / result panels** — short-lived floating windows
4. **Settings / Library / Notes browser** — standard app windows

This keeps invoke latency low without keeping large UI always visible.

## Data flow: one selection

```text
Hotkey pressed
  → Overlay shown
  → Region confirmed
  → Capture writes temp PNG (session-scoped path)
  → OCR runs locally (parallel)
  → Optional vision classify (if configured / needed)
  → Intent Engine builds ActionSet
  → Toolbar rendered near selection
  → User picks action
  → Action executor (may call network)
  → Result panel
  → On dismiss: delete temp capture unless saved to note/history policy
```

## Cross-cutting concerns

| Concern | Approach |
|---------|----------|
| Latency | Local OCR first; classify async; don’t block toolbar on slow vision |
| Offline | Math + OCR + Copy + Library/Notes browse & search work without network |
| Secrets | API keys in OS keyring or encrypted settings store |
| Observability | Local debug logs; opt-in anonymous product metrics later |
| Errors | User-visible recoverable errors; never leave orphan temp files |

## Non-functional targets (MVP)

| Metric | Target (stretch) |
|--------|------------------|
| Hotkey → overlay visible | < 100 ms |
| Selection confirm → toolbar | < 500 ms (OCR may stream in) |
| Local math result | < 100 ms |
| Temp screenshot lifetime | Session / until action complete, then delete unless retained |

## Related ADRs

- [ADR-001 Desktop framework](../decisions/ADR-001-desktop-framework.md)
- [ADR-002 OCR](../decisions/ADR-002-ocr.md)
- [ADR-003 Database](../decisions/ADR-003-database.md)
- [ADR-004 AI abstraction](../decisions/ADR-004-ai-abstraction.md)
