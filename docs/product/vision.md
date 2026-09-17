# Product vision

**Status:** Accepted  
**Product name:** Scoop

## One-line definition

> Scoop is an AI-powered Linux desktop utility that lets you select anything on your screen and instantly search it, calculate it, understand it, turn it into code, or save it as a note.

## Core interaction

```text
Select → Understand → Act → Save
```

Scoop should feel like an **AI layer sitting above the Linux desktop**, not another app users have to open and manage.

## Habit loop

```text
See something
     ↓
Select it
     ↓
Scoop
     ↓
Do something with it
```

## Differentiation

Scoop is not “AI for Linux.”

It is a **universal visual interaction layer for the desktop**.

Differentiation comes from:

1. **Speed** — screen to useful result in seconds
2. **Selection-first UX** — start from what’s already visible
3. **Contextual actions** — right tool for the selected content
4. **Minimal context switching** — floating results, not app hopping

## Product principle

Every feature must pass this test:

> Can the user get from something visible on their screen to a useful result in seconds?

| Pass | Fail |
|------|------|
| Select → act → result | Open app → copy → paste → configure → wait |

If a workflow requires opening multiple applications, copying content, pasting elsewhere, and manual configuration — Scoop should eliminate those steps.

## Design priorities (ordered)

1. Interaction quality of the marquee + toolbar
2. Correct capture across monitors and DPI
3. Reliable OCR for code/errors/math
4. Fast, useful default actions
5. Privacy-respecting defaults
6. Breadth of AI features

Prefer fewer features that feel instant over many features that feel heavy.

## Non-goals (MVP)

- Becoming a full IDE, browser, or note-taking suite
- Replacing specialized math/CAS tools for advanced work
- Team collaboration / shared workspaces
- Non-Linux platforms
- Plugin marketplace
