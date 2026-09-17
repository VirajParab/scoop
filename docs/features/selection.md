# Feature: Global hotkey & marquee selection

**Status:** Spec accepted  
**Priority:** P0  
**Related:** [desktop.md](../architecture/desktop.md)

## Summary

Users invoke Scoop from anywhere with a global hotkey, then drag a rectangle over any on-screen content.

## User flow

```text
Global Hotkey
  → Selection mode (dimmed overlay)
  → Drag rectangle
  → Optional resize
  → Confirm (Enter / release per UX)
  → Capture + continue pipeline
  → Escape cancels at any time before confirm
```

## Hotkey requirements

| Req | Detail |
|-----|--------|
| Global | Works regardless of active application (platform permitting) |
| Default | `Super + Shift + Space` (configurable) |
| Conflict detection | Warn when binding fails or conflicts when detectable |
| Disable | User can turn off global hotkey |
| Reassign | Settings UI to record new chord |

## Marquee requirements

- Highlight selected region; dim background (toggleable)
- Resize handles before submission
- Show dimensions while dragging
- Escape cancels; restore previous focus when possible
- Works over browser, terminal, IDE, PDF, video, image, desktop apps

## Multi-monitor (MVP)

- App works on multi-monitor desktops
- A single selection is confined to **one monitor** (no cross-monitor stitch)

## Acceptance criteria

- [ ] Hotkey triggers overlay within latency target
- [ ] Cancel leaves no overlay or stuck grab
- [ ] Confirm yields correct region to capture module
- [ ] Settings change applies without restart when possible
- [ ] Documented limitations per compositor

## Edge cases

- Modifier key stuck after cancel
- Hotkey pressed while Scoop settings focused
- Fullscreen games (may fail — document)
- Secure attention / lock screen (must not capture)
