# ADR-003: Local database

**Status:** Accepted  
**Date:** 2026-09-17

## Context

Scoop must persist a searchable **library** (screenshots & text), notes, history metadata, and settings locally for MVP. Data volume is modest; search starts as keyword-only.

## Decision

Use **SQLite** with:

- Schema migrations on startup
- **FTS5** for library + notes keyword search
- Media files on disk; paths in DB
- Non-secret settings in DB; secrets in OS keyring

Locations follow XDG (`$XDG_DATA_HOME/scoop/`).

## Consequences

### Positive

- Zero ops; single file backup possible later
- Good enough performance for personal library/notes
- FTS5 covers MVP search

### Negative

- Semantic search needs a later store/index (v2)
- Multi-device sync not inherent

### Follow-ups

- Define `001_init.sql` with collections, library_items, notes, history, settings, FTS
- Sweeper for retention policies

## Alternatives rejected

| Store | Why not |
|-------|---------|
| JSON files only | Weak query/search |
| PostgreSQL | Overkill for desktop utility |
| IndexedDB in webview | Poor fit for Rust core / files |
