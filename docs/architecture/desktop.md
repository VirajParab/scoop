# Desktop & Linux integration

**Status:** Accepted (directional)  
**Priority:** P0 foundation

## Responsibilities

1. Register and handle **global hotkeys**
2. Present a **fullscreen selection overlay**
3. Capture the **exact pixel region** across monitors and scale factors
4. Manage **window stacking** so overlay sits above other apps
5. Integrate with **clipboard** and optional **autostart**

## Display server support

| Server | MVP expectation |
|--------|-----------------|
| **X11** | Full support |
| **Wayland** | Full support on common compositors (GNOME, KDE, wlroots-based) |

Capture and global shortcuts differ significantly between X11 and Wayland. Abstract behind:

```text
trait DesktopBackend {
  register_hotkey(...)
  grab_region_screenshot(rect) -> Image
  set_clipboard(text | image)
  get_active_window_meta() -> Option<WindowMeta>  // v2
}
```

Implementations: `X11Backend`, `WaylandBackend` (portal / compositor protocols as required).

## Global hotkey

Default (example): `Super + Shift + Space`

Requirements:

- Works regardless of focused app (within OS capabilities)
- Configurable in settings
- Detect conflicts where the platform allows
- Disable / reassign without reinstall

**Wayland note:** Prefer portal / compositor-supported global shortcut APIs where available; document compositor limitations in troubleshooting.

## Marquee selection UX

```text
Invoke → Overlay (optional dim) → Drag rect → Optional resize handles
       → Enter / click confirm → Escape cancel
```

Rules:

- Coordinates in **physical pixels** for capture; UI in logical pixels
- Show size label (WxH) during drag
- Cross-monitor selection: either clamp to one monitor (MVP) or stitch (stretch) — **MVP: single-monitor selections only; multi-monitor desktop supported as separate surfaces**
- Cursor: crosshair; Escape always cancels and restores focus when possible

## Screen capture requirements

| Requirement | Detail |
|-------------|--------|
| Region screenshot | Exact selection bounds |
| HiDPI | Use correct scale per monitor |
| Multi-monitor | Correct origin mapping |
| Resolution | Preserve native pixels (no unnecessary downscale) |
| Storage | Write under `$XDG_CACHE_HOME/scoop/captures/` (or equivalent) |
| Cleanup | Delete on cancel, dismiss, or TTL; retain only if user saves |

Privacy: never promote temp captures to permanent storage without explicit Save Note / user retention settings.

## Clipboard

- Copy **text** results via primary/clipboard as appropriate for Linux UX
- Prefer standard clipboard (`CLIPBOARD`) for Copy actions
- Optional: also set `PRIMARY` only if it matches user expectation — default off to avoid surprising paste-on-middle-click

## Autostart

Optional “Launch on startup” via XDG autostart `.desktop` entry.

## Sensitive apps (v2)

When exclusions ship: if active window matches exclusion list, hotkey no-ops or shows a brief “disabled for this app” toast.

## Testing matrix (minimum)

| Distro / DE | Display | Priority |
|-------------|---------|----------|
| Ubuntu + GNOME | Wayland | P0 |
| Fedora + GNOME | Wayland | P0 |
| KDE Plasma | Wayland | P0 |
| Any + X11 session | X11 | P0 |
| Sway / Hyprland | Wayland | P1 |

## Related

- Feature: [selection.md](../features/selection.md)
- Feature: [capture-ocr.md](../features/capture-ocr.md)
- ADR: [ADR-001](../decisions/ADR-001-desktop-framework.md)
