# Feature: HTML generation & preview (v2)

**Status:** Deferred to v2  
**Priority:** Post-MVP differentiator  
**Related:** [roadmap.md](../product/roadmap.md)

## Summary

User selects a UI screenshot → **Generate HTML** → Scoop produces HTML/CSS (JS if needed) → live preview → iterative AI edits.

## MVP product note

Not in v1. Keep architecture ready:

- `AiProvider::generate_html`
- Action id reserved: `generate_html`
- Content type `UI_SCREENSHOT` already classified

## Target experience

```text
┌────────────────────────────────────────────┐
│ Generated UI                               │
├────────────────────────────────────────────┤
│                                            │
│        LIVE HTML PREVIEW                   │
│                                            │
├────────────────────────────────────────────┤
│ Copy Code │ Save │ Open Browser │ Edit AI │
└────────────────────────────────────────────┘
```

Iterative edits:

> “Make the cards smaller.”  
> “Use a dark theme.”  
> “Add a navigation bar.”  
> “Make the sidebar 20% narrower.”

## Framework options

| Phase | Support |
|-------|---------|
| v2 initial | **HTML + CSS** |
| v2 stretch | React, Tailwind, Vue |

## Acceptance criteria (when scheduled)

- [ ] Generates viewable HTML/CSS from UI screenshot
- [ ] Preview sandboxed (no unexpected network from preview)
- [ ] Copy code / save bundle / open in browser
- [ ] Follow-up edit prompts modify code coherently
- [ ] Save Note can attach code + screenshot

## Security

Preview in isolated webview with locked-down permissions. Treat model output as untrusted HTML.
