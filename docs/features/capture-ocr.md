# Feature: Screen capture & OCR

**Status:** Spec accepted  
**Priority:** P0  
**Related:** [privacy.md](../architecture/privacy.md) · [ADR-002](../decisions/ADR-002-ocr.md)

## Summary

After marquee confirm, Scoop captures the region and extracts text via OCR. OCR text is editable before actions that consume it.

## Capture requirements

| Capability | Requirement |
|------------|-------------|
| Region screenshot | Exact selection |
| HiDPI | Correct per-monitor scale |
| Multi-monitor | Correct coordinate mapping |
| Resolution | Preserve native pixels |
| Temp storage | `$XDG_CACHE_HOME/scoop/captures/` |
| Cleanup | Auto-delete unless user saves |
| Privacy | No permanent store by default |

## OCR requirements

Support initially:

- English
- Multiple fonts
- Dark and light backgrounds
- Code, tables, numbers
- Mathematical notation where possible

Pipeline:

```text
Image → preprocess (optional) → OCR → editable text buffer
```

User can edit OCR text in the toolbar/panel before Search, Ask AI, or Save Note.

## Parallelism

- Start OCR immediately after capture
- Show toolbar with universal actions before OCR completes when possible; fill text when ready
- Vision classify may run in parallel; must not block local actions (Copy of image path excepted)

## Acceptance criteria

- [ ] Captured pixels match selection on 100% and 200% scale fixtures
- [ ] Temp file removed after cancel/dismiss (sweeper verifies)
- [ ] OCR editable and edits flow into Search / Ask AI / Note
- [ ] Failure shows `OCR_FAILED` with retry; raw image actions still available

## Non-goals (MVP)

- Handwriting specialist models
- Multi-language packs beyond EN (architecture should allow adding langs)
- Cloud OCR as default
