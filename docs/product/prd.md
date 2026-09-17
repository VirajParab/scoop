# Product requirements & feature specification

**Document type:** Product Requirements & Feature Specification  
**Status:** MVP Planning  
**Target platform:** Linux Desktop  
**Product name:** Scoop

---

## 1. Product vision

Scoop is a Linux desktop application that allows users to **select anything visible on their screen and immediately perform useful actions on it**.

Core interaction:

> **Select → Understand → Act → Save**

Instead of switching between screenshots, browsers, calculators, AI tools, note apps, and development tools, the user selects a region and invokes an appropriate action.

Examples:

| Selection | Action |
|-----------|--------|
| Mathematical expression | Calculate |
| Article / paragraph | Search / Summarize |
| UI screenshot | Generate HTML/CSS (v2) |
| Useful information | Save as Note |

Scoop should feel like an **AI layer above the Linux desktop**.

See also: [vision.md](./vision.md)

---

## 2. Target users

See [personas.md](./personas.md).

Primary audiences: **developers**, **students**, **researchers**, **knowledge workers**.

Initial go-to-market focus: **Linux developers + technical power users**.

---

## 3. Core product concept

Built around a **global screen-selection interaction**.

### User flow

```text
Global Hotkey
      ↓
Screen freezes / selection mode
      ↓
User draws rectangle
      ↓
Scoop captures selected region
      ↓
OCR + Vision + Context Detection
      ↓
Action Toolbar
      ↓
User chooses action
      ↓
Result
      ↓
Copy / Save / Continue
```

---

## 4. MVP feature list

### P0 — Must have

#### 4.1 Global hotkey

Invoke Scoop from anywhere.

Example: `Super + Shift + Space`

Requirements:

- Works regardless of currently active application
- Configurable hotkey
- Hotkey conflict detection
- Option to disable / reassign

Spec: [selection.md](../features/selection.md)

#### 4.2 Visual marquee selector

After invoke:

- Screen enters selection mode
- User drags a rectangle
- Selected region highlighted; background dimmable
- Selection resizable before submission
- Escape cancels

Must work across: browser, terminal, IDE, PDF viewer, video, image, desktop apps.

---

## 5. Screen capture

Required:

- Screenshot of selection
- High-DPI support
- Multi-monitor support
- Correct monitor scaling
- Preserve original resolution
- Temporary local storage
- Automatic cleanup

Privacy:

> Screenshots must not be permanently stored unless the user explicitly saves them.

Spec: [capture-ocr.md](../features/capture-ocr.md)

---

## 6. OCR

Extract text from selected regions.

Support (MVP):

- English initially
- Multiple fonts
- Dark/light backgrounds
- Code, tables, numbers
- Mathematical notation where possible

OCR output should be editable before certain actions.

---

## 7. AI vision understanding

When needed, send the selection to a vision model and classify content:

```text
TEXT | CODE | MATH | TABLE | CHART | IMAGE
UI_SCREENSHOT | PRODUCT | DOCUMENT | UNKNOWN
```

Classification drives action suggestions.

| Type | Suggested actions |
|------|-------------------|
| Code | Explain, Debug, Search, Copy, Save Note |
| Math | Calculate, Explain, Solve, Copy, Save Note |
| UI | Generate HTML (v2), Explain, Extract Text, Save Note |

---

## 8. Contextual action bar

Compact floating toolbar after selection. Dynamically adapts to content type.

### Universal actions

- Ask AI
- Search
- Copy
- Save to Library
- Save Note

### Context-specific (examples)

| Domain | Actions |
|--------|---------|
| Math | Calculate, Solve, Explain |
| Code | Explain, Debug, Refactor, Generate docs |
| UI | Generate HTML, Generate React, Generate CSS (v2) |
| Image | Describe, Extract text, Search image |
| Table | Analyze, Extract, Calculate, Convert to CSV |

Spec: [actions.md](../features/actions.md)

---

## 9. Web search

Search selected content via configurable provider (Google / Bing / DuckDuckGo).

Modes:

1. **Exact search** — OCR text as query
2. **AI search query** — AI rewrites into a better query

Spec: [search.md](../features/search.md)

---

## 10. Math engine

Recognize and evaluate expressions, e.g.:

```text
₹1,25,000 × 12%
2^10
sqrt(144)
15% of 4,500
(12 × 45) / 3
```

Output: Expression, Calculation, Answer.

Prefer a **local deterministic math engine** when possible; use AI for ambiguous/complex problems.

Spec: [math.md](../features/math.md)

---

## 11. AI assistant

Ask questions about selected content. Inputs to the model:

- Screenshot
- OCR text
- Detected content type
- Optional application context
- User question

Results appear in a **floating panel**.

Spec: [ai-assistant.md](../features/ai-assistant.md)

---

## 12–13. HTML generation & preview (v2)

Differentiating feature deferred to v2: generate HTML/CSS from UI screenshots with live preview and iterative AI edits.

Spec: [html-generation.md](../features/html-generation.md)

---

## 14. Local library (screenshots & copied text)

Users can **save, arrange, and search** screenshots and copied text locally anytime.

- Save screenshot + OCR/copied text from the toolbar
- Arrange into **collections** (folders) and tags
- Keyword search across titles, text, OCR, and tags — fully offline
- Distinct from short-lived History; explicit keep aligned with privacy defaults

Spec: [library.md](../features/library.md)

---

## 15–16. Notes

Core feature, not an afterthought. Complements the library: notes are structured documents; the library is the durable capture shelf.

Basic note fields: title, content, screenshot, extracted text, source URL, created date, tags.

**Smart notes:** optional AI structuring (title, summary, key points, facts, tags).

**Notes search (MVP):** keyword search. Semantic / NL search in later versions.

Spec: [notes.md](../features/notes.md)

---

## 17. Clipboard

Copy results after any operation (OCR, math answer, AI response, HTML, search query). Native Linux clipboard integration. Optional: also save copied text into the Library (off by default).

---

## 18. History

Recent actions with screenshot, input, action, output, timestamp. User can delete.

Spec: [history.md](../features/history.md)

---

## 19. Privacy & security

Default: do not permanently retain selected screen content unless the user saves it.

Settings for AI provider, API keys, cloud vs local processing, history retention, auto-delete screenshots.

Future: exclude sensitive apps (password managers, banking, private windows).

Spec: [privacy.md](../architecture/privacy.md)

---

## 20. AI provider architecture

Do not hard-code one vendor. Provider interface with adapters for OpenAI, Anthropic, Google, local models.

Spec: [ai-providers.md](../architecture/ai-providers.md)

---

## 21–22. Linux architecture & components

See [overview.md](../architecture/overview.md) and [tech-stack.md](../engineering/tech-stack.md).

Suggested flow: Hotkey → Marquee → Capture → OCR / Vision → Intent Engine → Search / Math / AI → Result Layer → Copy / Library / Notes / Preview.

---

## 23. Settings

General, AI, Search, Notes, Privacy. Full list in [settings.md](../features/settings.md).

---

## 24. MVP scope

See [mvp-scope.md](./mvp-scope.md) and [roadmap.md](./roadmap.md).

---

## 25. Example scenarios

### A — Developer error

Selection: `connection refused 127.0.0.1:5432` → Explain | Search | Fix | Copy | Save Note.

### B — Mathematics

Selection: `₹2,40,000 × 8.5% × 4` → Calculate → `₹81,600`.

### C — Research

Selection: research paragraph → Ask AI | Search | Summarize | Save Note (structured).

### D — UI recreation (v2)

Selection: website UI → Generate HTML → live preview → iterative edits.

---

## 26. Business model (directional)

| Tier | Direction |
|------|-----------|
| Free | Limited AI, local OCR, calculator, basic notes, limited history |
| Pro (~$8–15/mo) | Higher AI limits, vision, HTML gen, AI notes, semantic search |
| Power | Local models, custom providers, automation, plugins |

Pricing validated via user testing — not fixed for MVP engineering.

---

## 27. Distribution

Linux packages: `.deb`, `.rpm`, AppImage, Flatpak. Channels: GitHub, Flathub, Snap, distro repos.

Initial audience: Linux developers + technical power users.

---

## 28–32. Differentiation, metrics, success, principles

See [vision.md](./vision.md) and [mvp-scope.md](./mvp-scope.md).
