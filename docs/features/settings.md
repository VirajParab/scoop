# Feature: Settings

**Status:** Spec accepted  
**Priority:** P0

## Summary

Central preferences for general behavior, AI, search, library, notes, and privacy.

## Sections

### General

- Launch on startup
- Global hotkey (record chord)
- Selection behavior (dim intensity, confirm mode)
- Language (UI; OCR lang later)

### AI

- AI provider
- API key (secret store)
- Model (text / vision)
- Temperature where applicable
- Cloud processing enable

### Search

- Search provider (Google / Bing / DuckDuckGo)
- Default browser (system default vs explicit binary — stretch)
- Default search mode (Exact vs AI query)

### Library

- Default collection for quick-save (`Inbox` or last used)
- Also save copied text to Library (on/off, default off)
- Show screenshot thumbnails in browse (on/off)

### Notes

- Storage location (advanced; default XDG — shared with library)
- Auto-tagging (on/off)
- Screenshot attachment default
- Link new notes to a library item when both are saved (on/off, default on)

### Privacy

- Application exclusions (v2 UI; stub OK)
- History retention (days)
- Auto-delete screenshots / history media
- Cloud processing (mirror of AI toggle)

## UX requirements

- Validate hotkey; show conflict errors
- Test connection button for AI provider
- Never show full API key after save (mask; allow rotate)
- Changes apply immediately when safe

## Acceptance criteria

- [ ] All MVP settings persist across restart
- [ ] API key readable by provider layer, not by logs
- [ ] Disabling cloud disables AI actions in toolbar
- [ ] Hotkey rebind works without reinstall

## Related

- [privacy.md](../architecture/privacy.md)
- [ai-providers.md](../architecture/ai-providers.md)
