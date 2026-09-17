# Testing strategy

**Status:** Draft

## Goals

1. Protect the **select → understand → act** loop from regressions
2. Catch **DPI / multi-monitor / Wayland vs X11** capture bugs early
3. Keep provider adapters covered without live paid API calls on every CI run

## Test layers

| Layer | What | Where |
|-------|------|-------|
| Unit | Math parser, heuristics classify, FTS queries, retention sweeper | `cargo test` / UI unit |
| Component | Toolbar action ranking, settings validation | UI + core |
| Integration | Capture → OCR pipeline with fixture images | Local + CI |
| Manual / QA | Hotkey, overlay, multi-monitor | Matrix in desktop.md |
| Contract | AI provider adapters with recorded fixtures | Mock HTTP |

## Fixture images

Maintain a small corpus under `testdata/ocr/`:

| Fixture | Expectation |
|---------|-------------|
| `code_dark_terminal.png` | Error lines readable |
| `math_currency.png` | Expression OCR + math parse |
| `serif_pdf_light.png` | Prose paragraph |
| `table_simple.png` | Cell text roughly extracted |
| `ui_dashboard.png` | Buttons/labels (for v2 HTML) |

CI asserts OCR contains key substrings (fuzzy where needed).

## Math tests

Golden cases from PRD:

```text
2^10 → 1024
sqrt(144) → 12
15% of 4500 → 675
(12 * 45) / 3 → 180
```

Plus Indian currency grouping (`₹1,25,000`) and `%` / `×` normalization.

## Provider tests

- Replay recorded JSON responses (no network)
- Weekly optional live smoke job with secret (manual/nightly)

## Manual QA checklist (each release candidate)

- [ ] Hotkey invoke on GNOME Wayland
- [ ] Hotkey invoke on X11 session
- [ ] Dual monitor: select on each display, verify pixels
- [ ] HiDPI 150%/200% scale
- [ ] Escape cancels; no leftover overlay
- [ ] Temp capture deleted after dismiss
- [ ] Save Note retains screenshot; cancel does not
- [ ] Exact Search opens configured provider
- [ ] Ask AI with missing key shows setup, not crash
- [ ] History delete removes row (+ media if any)

## Performance smoke

Log timings in debug builds:

- hotkey → overlay
- confirm → toolbar
- OCR duration
- classify duration

Fail CI only on extreme regressions once baselines exist.

## Accessibility smoke

Keyboard-only: invoke, select via mouse (required), navigate toolbar with arrows/enter, Escape dismiss.
