# ADR-001: Desktop application framework

**Status:** Accepted  
**Date:** 2026-09-17  
**Deciders:** Engineering

## Context

Scoop needs a lightweight Linux desktop app with:

- Global hotkeys
- Fullscreen selection overlay
- Region capture (X11 + Wayland)
- Floating toolbars and settings UI
- Small resource footprint

Options considered: Tauri, Electron, GTK, Qt, pure Rust GUI.

## Decision

**Prefer Tauri 2 (Rust core + webview UI)** for MVP, with the option to implement the selection overlay as a native companion window if webview overlays prove unreliable on Wayland.

## Consequences

### Positive

- Smaller binaries than Electron
- Rust for capture/OCR/DB/providers
- Fast UI iteration for panels/settings

### Negative / risks

- Wayland global shortcut and overlay quirks
- Webview theming vs fully native look
- Extra moving parts vs pure GTK

### Follow-ups

- Spike hotkey + overlay on GNOME/KDE Wayland in week 1
- If blocked, evaluate GTK overlay + Tauri panels hybrid

## Alternatives not chosen

| Option | Why not (for now) |
|--------|-------------------|
| Electron | Heavier; weaker “utility” feel |
| GTK/Qt only | Slower panel iteration |
| egui/iced | Weaker docs/settings UX velocity |
