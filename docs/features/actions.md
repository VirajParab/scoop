# Feature: Contextual action bar

**Status:** Spec accepted  
**Priority:** P0  
**Related:** [ai-providers.md](../architecture/ai-providers.md)

## Summary

After selection, Scoop shows a compact floating toolbar. Actions adapt to detected content type.

## Layout (example)

```text
┌──────────────────────────────────────────────────┐
│ Search │ Calculate │ Ask AI │ Library │ Note │ Copy │
└──────────────────────────────────────────────────┘
```

Position near the selection without covering it when possible; clamp to monitor work area.

## Content types

```text
TEXT | CODE | MATH | TABLE | CHART | IMAGE
UI_SCREENSHOT | PRODUCT | DOCUMENT | UNKNOWN
```

## Action sets

### Universal (always available)

- Ask AI
- Search
- Copy
- **Save to Library**
- Save Note

### Context-specific

| Type | Actions (MVP bold) |
|------|--------------------|
| Math | **Calculate**, Solve, Explain |
| Code | **Explain**, Debug, Search, Copy, Save to Library, Save Note |
| UI | Explain, Extract Text, Save to Library, Save Note (Generate HTML = v2) |
| Image | Describe, Extract text, Search, Save to Library |
| Table | Analyze, Extract, Calculate |
| Text / Document | Summarize (via Ask AI), Search, Save to Library, Save Note |

MVP ships the **bold** subset plus universal actions; additional verbs can map to Ask AI prompts.

## Intent engine

Inputs: OCR text, optional vision type, heuristics.

Output: ordered `ActionId` list (max ~5–7 visible; overflow menu if needed).

Rules:

1. Never wait > N ms for vision before first paint of universal actions
2. Patch toolbar when classification arrives
3. Prefer Calculate when math heuristics fire strongly

## Acceptance criteria

- [ ] Toolbar appears after confirm with universal actions
- [ ] Math selection surfaces Calculate prominently
- [ ] Code/error selection surfaces Explain / Search
- [ ] Keyboard: arrow keys + Enter activate; Escape dismisses
- [ ] Clicking outside can dismiss (configurable)

## UX constraints

- One composition: toolbar is a utility chrome, not a dashboard
- No card grids of features in the selection moment
- Results open in a separate floating panel, not nested mega-menus
