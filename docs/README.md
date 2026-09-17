# Scoop documentation

Documentation for **Scoop** — an AI-powered visual workspace for Linux.

## Product

| Doc | Purpose |
|-----|---------|
| [vision.md](./product/vision.md) | Vision, principles, differentiation |
| [prd.md](./product/prd.md) | Full product requirements & feature spec |
| [personas.md](./product/personas.md) | Target users and use cases |
| [mvp-scope.md](./product/mvp-scope.md) | MVP cut line, success criteria, metrics |
| [roadmap.md](./product/roadmap.md) | MVP → v2 → v3 |
| [glossary.md](./product/glossary.md) | Shared terms |

## Architecture

| Doc | Purpose |
|-----|---------|
| [overview.md](./architecture/overview.md) | End-to-end system architecture |
| [desktop.md](./architecture/desktop.md) | Hotkey, selection, capture, Linux integration |
| [ai-providers.md](./architecture/ai-providers.md) | Provider abstraction & interfaces |
| [data-model.md](./architecture/data-model.md) | SQLite schema, library, notes, history, settings |
| [privacy.md](./architecture/privacy.md) | Privacy defaults, retention, exclusions |

## Engineering

| Doc | Purpose |
|-----|---------|
| [tech-stack.md](./engineering/tech-stack.md) | Recommended stack & rationale |
| [development-setup.md](./engineering/development-setup.md) | Local setup, tooling, runbook |
| [coding-standards.md](./engineering/coding-standards.md) | Conventions & module boundaries |
| [testing.md](./engineering/testing.md) | Test strategy & environments |
| [qa-checklist.md](./engineering/qa-checklist.md) | Manual QA / release checklist |

## Feature specs

| Doc | Priority | Purpose |
|-----|----------|---------|
| [selection.md](./features/selection.md) | P0 | Global hotkey + marquee |
| [capture-ocr.md](./features/capture-ocr.md) | P0 | Screenshot + OCR |
| [actions.md](./features/actions.md) | P0 | Contextual action bar |
| [search.md](./features/search.md) | P0 | Web search |
| [math.md](./features/math.md) | P0 | Calculator / math engine |
| [ai-assistant.md](./features/ai-assistant.md) | P0 | Ask AI |
| [notes.md](./features/notes.md) | P0 | Notes, smart notes, search |
| [library.md](./features/library.md) | P0 | Arrange screenshots & text + search |
| [clipboard.md](./features/clipboard.md) | P0 | Clipboard integration |
| [history.md](./features/history.md) | P0 | Action history |
| [settings.md](./features/settings.md) | P0 | Settings & preferences |
| [html-generation.md](./features/html-generation.md) | v2 | HTML/CSS gen + preview |

## Decisions (ADRs)

| ADR | Topic |
|-----|-------|
| [ADR-001](./decisions/ADR-001-desktop-framework.md) | Desktop framework |
| [ADR-002](./decisions/ADR-002-ocr.md) | OCR engine |
| [ADR-003](./decisions/ADR-003-database.md) | Local database |
| [ADR-004](./decisions/ADR-004-ai-abstraction.md) | AI provider interface |

## Document status legend

- **Draft** — open for feedback
- **Accepted** — agreed direction for MVP
- **Deferred** — post-MVP
