# Feature: Clipboard integration

**Status:** Spec accepted  
**Priority:** P0

## Summary

After any operation, users can copy results to the Linux clipboard.

## Copy sources

| Source | Clipboard payload |
|--------|-------------------|
| OCR | Extracted / edited text |
| Math | Answer (optionally expression) |
| Ask AI | Full or selection of response |
| Search | Final search query |
| HTML (v2) | Generated code |
| Note | Title / body on demand |
| Library | Clip text / OCR from a library item |

## Behavior

- Use the standard `CLIPBOARD` selection for explicit Copy actions
- Do not overwrite clipboard on every selection automatically
- Optional setting: “Copy OCR text on selection confirm” (default **off**)
- Optional setting: “Also save copied text to Library” (default **off**) — when on, Copy creates/updates a text library item in `Inbox`

## Acceptance criteria

- [ ] Copy works from result panels, toolbar, and library detail
- [ ] Paste into terminal/editor receives expected text
- [ ] Wayland and X11 verified on QA matrix
- [ ] Errors surface if clipboard backend missing (`wl-clipboard` / similar)
- [ ] Opt-in “save copy to Library” respects privacy defaults (off by default)
