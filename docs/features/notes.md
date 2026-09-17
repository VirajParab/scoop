# Feature: Notes, smart notes & search

**Status:** Spec accepted  
**Priority:** P0 (notes are core, not an afterthought)  
**Related:** [data-model.md](../architecture/data-model.md)

## Summary

Save any selection as a note (with optional screenshot and OCR). Optionally let AI structure the note. Browse and keyword-search saved notes.

For arranging raw screenshots and copied text for anytime search, see the **[Local library](./library.md)** — Notes are structured documents; Library is the durable capture shelf.

## Basic note fields

```text
Title
Content
Screenshot
Extracted text
Source URL
Created date
Tags
```

Example:

```text
Title:   Postgres Serialization Errors
Source:  stackoverflow.com
Content: ...
Screenshot: [attached]
Tags: postgres, database, backend
```

## Smart notes (optional)

On Save Note, user can enable **Smart structure**:

```text
Title
Summary
Key Points
Important Facts
Source
Tags
Screenshot
Original Selection
```

User edits everything before commit. If AI fails → fall back to basic note draft.

## Notes browser

Simple list + detail:

- Sort by created date
- Filter by tag (stretch)
- Open screenshot
- Edit / delete

## Notes search (MVP)

**Keyword search** via SQLite FTS5 over title, content, OCR text, tags, summary.

Examples of queries users will try:

```text
postgres
things I saved about React
Kubernetes
authentication
```

Natural-language / semantic search → **v2**.

## Acceptance criteria

- [ ] Save from toolbar creates editable draft then persists
- [ ] Screenshot attached only if user keeps it (default on for notes)
- [ ] Smart notes optional and editable pre-save
- [ ] Keyword search returns expected notes on fixtures
- [ ] Delete note removes DB row and media file

## Non-goals (MVP)

- Sync / multi-device
- Shared team notebooks
- Bidirectional web clipper extension

## Relationship to Library

Prefer linking a note to a `library_item_id` when the same screenshot is kept in both places, so media is not duplicated on disk.
