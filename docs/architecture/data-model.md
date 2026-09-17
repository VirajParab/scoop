# Data model

**Status:** Accepted  
**Store:** SQLite (see [ADR-003](../decisions/ADR-003-database.md))

## Storage layout

| Path | Contents |
|------|----------|
| `$XDG_DATA_HOME/scoop/scoop.db` | Library, notes, history metadata, settings (non-secret) |
| `$XDG_DATA_HOME/scoop/media/` | Persisted screenshots for library items / notes / history |
| `$XDG_CACHE_HOME/scoop/captures/` | Temporary selection captures |
| OS keyring / secret service | API keys |

Respect XDG Base Directory Spec on Linux.

## Entities

### collections

User-arranged folders for the local library. Seed with system collection `Inbox`.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK (UUID) | |
| name | TEXT | Unique display name |
| is_system | INTEGER | 1 for `Inbox` |
| sort_order | INTEGER | |
| created_at | TEXT (ISO-8601) | |
| updated_at | TEXT (ISO-8601) | |

### library_items

Durable screenshots and/or copied text — searchable anytime. See [library.md](../features/library.md).

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK (UUID) | |
| collection_id | TEXT FK → collections | Default Inbox |
| title | TEXT | |
| item_type | TEXT | `screenshot` \| `text` \| `mixed` |
| clip_text | TEXT NULL | Copied / edited text body |
| ocr_text | TEXT NULL | OCR at save time |
| tags | TEXT | JSON string array |
| screenshot_path | TEXT NULL | Relative under media/ |
| content_type | TEXT NULL | CODE, MATH, … |
| source_url | TEXT NULL | |
| note_id | TEXT NULL | Optional link to notes.id |
| created_at | TEXT (ISO-8601) | |
| updated_at | TEXT (ISO-8601) | |

Indexes: `collection_id`, `created_at DESC`; FTS on title/clip_text/ocr_text/tags.

### notes

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK (UUID) | |
| title | TEXT | |
| content | TEXT | Body / markdown |
| summary | TEXT NULL | Smart notes |
| key_points | TEXT NULL | JSON array |
| important_facts | TEXT NULL | JSON array |
| ocr_text | TEXT NULL | |
| source_url | TEXT NULL | |
| tags | TEXT | JSON string array |
| screenshot_path | TEXT NULL | Relative under media/ (or shared via library_item_id) |
| library_item_id | TEXT NULL | Optional FK → library_items to avoid duplicate media |
| content_type | TEXT NULL | Enum string |
| created_at | TEXT (ISO-8601) | |
| updated_at | TEXT (ISO-8601) | |
| is_smart | INTEGER | 0/1 |

Indexes: `created_at DESC`; FTS on title/content/ocr_text/tags (MVP keyword search).

### history

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | |
| action | TEXT | `search`, `calculate`, `ask_ai`, `save_library`, `save_note`, `copy`, … |
| input_summary | TEXT | Truncated OCR / query |
| output_summary | TEXT | Truncated result |
| screenshot_path | TEXT NULL | Only if retention allows |
| content_type | TEXT NULL | |
| created_at | TEXT | |
| meta_json | TEXT NULL | Action-specific payload |

### settings

Simple key-value or JSON blob table:

| Column | Type |
|--------|------|
| key | TEXT PK |
| value | TEXT |

Non-secret settings only (hotkey, provider id, model names, retention days, search provider, etc.).

### tags (optional normalized)

MVP may keep tags as JSON on `notes` and `library_items`. Normalize in v2 if filtering needs it.

## Full-text search (MVP)

Use SQLite FTS5 for **both** notes and library:

```sql
-- Conceptual
CREATE VIRTUAL TABLE notes_fts USING fts5(
  title, content, ocr_text, tags, summary,
  content='notes', content_rowid='rowid'
);

CREATE VIRTUAL TABLE library_fts USING fts5(
  title, clip_text, ocr_text, tags, collection_name,
  content='library_items', content_rowid='rowid'
);
```

Unified “Search everything” UI may query both tables and merge ranked hits.

Semantic search is **out of MVP**.

## Retention & cleanup

| Asset | Default |
|-------|---------|
| Temp captures | Delete after action UI closes or TTL (e.g. 1h sweeper) |
| History screenshots | Off or short retention (user setting) |
| Library screenshots / text | Kept until user deletes the library item |
| Note screenshots | Kept while note exists (or via linked library item) |
| History rows | User-configurable days; manual delete |

Background sweeper on startup + periodic timer.

## Migrations

Use numbered SQL migrations (`001_init.sql`, …). App opens DB with migrate-on-start.

## Backup (future)

Export notes + library as zip (JSON + media). Not required for MVP.
