# Feature: Local library (screenshots & text)

**Status:** Spec accepted  
**Priority:** P0  
**Related:** [notes.md](./notes.md) · [data-model.md](../architecture/data-model.md) · [clipboard.md](./clipboard.md)

## Summary

Scoop keeps a **local library** of screenshots and copied text so users can **arrange**, **browse**, and **search** anything they saved — anytime, offline, without relying on chat history or temp captures.

This is intentional persistence (unlike History). Notes can still be richer structured documents; the library is the durable shelf for captures and clips.

## Problem

Without a library:

- Screenshots vanish after the selection session
- Copied text lives only in the OS clipboard until overwritten
- Users cannot later find “that error screenshot from last week”

## User goals

1. **Save** a selection’s screenshot and/or extracted/copied text in one click
2. **Arrange** items into folders (collections), tags, and optional titles
3. **Search** across image OCR text and saved text anytime
4. **Re-use** — open, copy again, attach to a note, or re-run Search / Ask AI

## Core concepts

| Concept | Meaning |
|---------|---------|
| **Library item** | One saved unit: screenshot, text clip, or both |
| **Collection** | User-named folder/group (e.g. `Postgres`, `Interview prep`) |
| **Clip text** | OCR text and/or explicitly copied result text stored with the item |
| **Library search** | Keyword search over titles, clip text, OCR, tags, collection names |

## Save entry points

From the action toolbar / result panels:

| Action | Behavior |
|--------|----------|
| **Save to Library** | Persist screenshot + current text buffer; prompt for title/collection/tags (quick-save uses defaults) |
| **Copy** | Copies to clipboard; optional setting “Also save copy to Library” (default **off**) |
| **Save Note** | Creates a note; may also link/create a library item for the same media |

Quick-save defaults:

- Title: first line of OCR / “Untitled capture” + time
- Collection: `Inbox` (system default) or last-used collection
- Always store OCR/edited text when available
- Screenshot kept when the item includes image capture

## Arrange

Users can:

- Create / rename / delete **collections**
- Move items between collections (including bulk)
- Add / edit **tags**
- Edit **title** and **clip text**
- Pin important items (stretch)
- Sort: newest, oldest, title, collection

MVP collections model: flat list of collections (no nested folders). Nested folders → v2 if needed.

## Browse UI

```text
┌─────────────────────────────────────────────────────────┐
│ Library                          [Search library…    ]  │
├──────────────┬──────────────────────────────────────────┤
│ Collections  │  Items                                   │
│ • Inbox      │  ┌────┐  Postgres refused…    today      │
│ • Work       │  │img │  tags: postgres                  │
│ • Study      │  └────┘                                  │
│ • All        │  ┌────┐  ₹2,40,000 × 8.5%…   yesterday   │
│              │  │txt │  collection: Finance             │
└──────────────┴──────────────────────────────────────────┘
```

Detail view: screenshot preview, full text, metadata, actions (Copy, Open Note, Ask AI, Search, Delete).

## Search (anytime)

**Local keyword search** (FTS5) over:

```text
title | clip_text | ocr_text | tags | collection name
```

Examples:

```text
postgres
CrashLoopBackOff
things about React hooks
authentication error
```

Requirements:

- Works **fully offline**
- Fast on thousands of items (personal scale)
- Results show thumbnail + text snippet + collection
- Semantic / NL search → v2 (same track as notes)

## Item types

| Type | Contents |
|------|----------|
| `screenshot` | Image + optional OCR text |
| `text` | Copied / pasted text only (no image) |
| `mixed` | Image + text (typical selection save) |

## Privacy

- Library storage is **local-only** by default
- Saving is always **explicit** (or opt-in “save copies”)
- Deleting an item removes DB row + media file
- Export/backup later; not required for MVP

Aligns with: screenshots are not kept unless the user saves them — **Save to Library** is that explicit keep.

## Acceptance criteria

- [ ] Save to Library from toolbar persists screenshot and/or text under `$XDG_DATA_HOME`
- [ ] Default `Inbox` collection exists
- [ ] User can create collections and move items
- [ ] Library search finds items by OCR/clip text and tags
- [ ] Works offline with no AI provider configured
- [ ] Delete removes media from disk
- [ ] History is separate; clearing history does not wipe Library

## Non-goals (MVP)

- Cloud sync of library
- Automatic screenshot of every selection
- Nested folder trees
- Duplicate image detection
- Full DAM / photo-manager features

## Relationship to Notes & History

| Store | Intent | Retention |
|-------|--------|-----------|
| **Library** | Arrange & search captures/clips | Until user deletes |
| **Notes** | Structured documents / smart notes | Until user deletes |
| **History** | Recent action log | Short / configurable |

A note may reference a library item id (optional link) so media is not duplicated when both are saved.
