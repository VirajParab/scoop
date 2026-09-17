# Manual QA checklist (MVP)

Use before tagging a release candidate.

## Capture loop

- [ ] Tray → **Select (Scoop)** opens overlay
- [ ] Global hotkey invokes overlay (or documented fallback works)
- [ ] Drag selection → toolbar appears
- [ ] Escape cancels overlay; no stuck window
- [ ] Temp capture deleted after dismiss toolbar

## Display

- [ ] Works on current session (`wayland` / `x11`)
- [ ] Dual monitor: select on each display when available
- [ ] HiDPI: selection roughly matches pixels

## Local actions

- [ ] OCR text editable (or manual entry if tesseract missing)
- [ ] Calculate on `2^10` → `1024`
- [ ] Exact Search opens configured provider
- [ ] Copy places text on clipboard
- [ ] Save to Library creates searchable item
- [ ] Save Note appears in Notes

## Persistence / privacy

- [ ] Library search finds saved OCR/clip text offline
- [ ] Clear History does not delete Library/Notes
- [ ] Delete library item removes media file

## AI / settings

- [ ] Missing API key → Ask AI shows clear error
- [ ] With key + cloud on → Ask AI returns answer
- [ ] Hotkey rebind persists after restart
- [ ] Cloud processing off disables AI actions with message

## Packaging smoke

- [ ] `npm run build` (frontend) succeeds
- [ ] `cargo test math_engine` passes
- [ ] `npm run tauri build` produces deb and/or AppImage when deps present
