# MVP scope & success criteria

**Status:** Accepted  
**Release:** MVP (v1)

## In scope (MVP)

```text
1.  Linux desktop application
2.  Global hotkey
3.  Marquee selection
4.  Screenshot capture
5.  OCR
6.  AI vision (content classification)
7.  Contextual toolbar
8.  Web search
9.  Calculator / math engine
10. Ask AI
11. Save Note (+ optional smart structure)
12. Local library (arrange screenshots & copied text + search)
13. Basic notes browser + keyword search
14. Clipboard integration
15. Basic history
16. Settings
```

## Explicitly out of MVP

| Feature | Target |
|---------|--------|
| HTML generation + live preview | v2 |
| AI editing of generated HTML | v2 |
| Semantic / NL notes search | v2 |
| Browser/source detection | v2 |
| Application context (active window metadata) | v2 |
| App exclusions for sensitive apps | v2 (design in privacy) |
| Local AI models | v3 |
| Plugins / automation | v3 |
| Team / shared notes | v3 |
| Distro packages beyond AppImage/deb stretch | v3 packaging push |

## Success criteria

The MVP must answer three questions:

### 1. Do people repeatedly use the marquee?

If users don’t repeatedly select things, the core interaction isn’t compelling.

**Proxy metrics:** selections per user/day; D1/D7 return with ≥1 selection.

### 2. Which action creates the most value?

Measure share and retention contribution of:

```text
Search | AI | Math | Notes | Library | Copy | OCR-only
```

### 3. Will power users pay?

Discover whether Scoop is useful enough to become a **daily desktop utility** worth a Pro subscription.

## Key metrics

### Activation

% of installs that complete first successful selection → action.

### Core usage

- Selections per user/day
- Actions per selection
- Repeat users
- Notes created
- Library items saved

### Feature usage

```text
Search | Calculate | Ask AI | Save Note | Library | OCR | Copy
```

(HTML tracked from v2.)

### Retention

Day 1, Day 7, Day 30.

### Business (post-monetization)

Free → Pro conversion, AI cost/user, revenue/user, MAU, churn.

## Definition of done (MVP)

- [x] Global hotkey registration (with tray fallback Select)
- [x] Multi-monitor region capture via `xcap` (verify on device QA)
- [x] OCR editable before Search / Ask AI / Note (tesseract CLI when installed)
- [x] Contextual toolbar shows type-appropriate actions
- [x] Search, Calculate, Ask AI, Save Note, Save to Library, Copy end-to-end
- [x] Temp screenshots cleaned unless saved to Library or Note
- [x] Library collections + keyword search (offline)
- [x] Notes keyword search
- [x] History list + delete (does not wipe Library)
- [x] Settings for hotkey, AI provider/key, search provider, retention
- [x] Privacy defaults documented and enforced in capture cleanup paths
- [ ] Full Wayland/X11 QA matrix signed off (see [qa-checklist.md](../engineering/qa-checklist.md))
- [ ] Packaged AppImage/`.deb` smoke on clean machine
