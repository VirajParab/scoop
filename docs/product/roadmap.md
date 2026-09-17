# Product roadmap

**Status:** Draft  
**Last updated:** 2026-09-17

## Timeline philosophy

Prove the **select → act** habit before expanding feature breadth. Ship a thin, fast MVP; deepen actions and generation in v2; local/offline and extensibility in v3.

---

## MVP (v1) — Habit formation

**Goal:** Prove repeated marquee usage and identify the highest-value actions.

| Area | Deliverables |
|------|----------------|
| Capture | Hotkey, marquee, multi-monitor HiDPI capture |
| Understand | OCR (EN), vision classification |
| Act | Search, Calculate, Ask AI, Copy |
| Persist | Notes (+ smart optional), **local library** (screenshots & text + search), keyword search, history |
| Config | Settings, provider abstraction, privacy defaults |
| Dist | Early AppImage / `.deb` for testers |

**Exit:** Metrics from [mvp-scope.md](./mvp-scope.md) show repeated use; at least one action clearly dominates value.

---

## Version 2 — Generation & context

**Goal:** Differentiated creative/rebuild workflows and smarter context.

```text
1. HTML generation (HTML + CSS first)
2. Live HTML preview
3. AI editing of generated HTML
4. Semantic search (notes + library)
5. Nested library collections
6. Better contextual actions
7. Browser / source detection
8. Application context (window title, app id)
9. Sensitive app exclusions
10. Framework options stretch: React / Tailwind / Vue (as capacity allows)
```

**Exit:** HTML gen used by power users; library/notes search quality improved; fewer wrong-action suggestions.

---

## Version 3 — Local, extensible, packaged

**Goal:** Power-user depth and broader Linux distribution.

```text
1. Local AI models
2. Advanced code generation
3. Workflow automation
4. Plugins
5. Multi-step actions
6. Team / shared notes
7. Linux distribution packages (Flatpak, rpm, Flathub, etc.)
8. Extension ecosystem
```

**Exit:** Local path viable without cloud; packaging on major channels; plugin surface stable enough for external contributors.

---

## Cross-cutting themes

| Theme | MVP | v2 | v3 |
|-------|-----|----|----|
| Privacy | Temp capture + retention settings | App exclusions | Local models / offline |
| AI providers | Cloud adapters + keys | More providers | Local backends |
| Performance | Sub-second selection UX | Faster classify | On-device inference |
| Distribution | Manual / AppImage / deb | Broader | Flathub + distro |

---

## Parking lot

Ideas not committed to a version:

- Video frame selection helpers
- Clipboard history integration
- Sync across machines
- Windows / macOS ports
- Voice-triggered selection
