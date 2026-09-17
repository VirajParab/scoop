# Feature: History

**Status:** Spec accepted  
**Priority:** P0

## Summary

Scoop maintains a recent actions list so users can revisit outputs without re-selecting.

## Example

```text
Today

12:32  Calculate
12:28  Search
12:21  Save to Library
12:18  Save Note
11:54  Ask AI
```

## Record fields

| Field | Notes |
|-------|-------|
| Screenshot | Optional; governed by retention settings |
| Input | Truncated OCR / query |
| Action | Enum / string id |
| Output | Truncated result |
| Timestamp | Local time |

## Operations

- Browse by day
- Open detail (full output when stored)
- Delete one / clear all
- Optional: re-run Search / Copy output

## Retention

Default: keep metadata for N days (e.g. 7–30); **screenshots off or short TTL**.

Sweeper deletes expired rows + orphan media.

## Acceptance criteria

- [ ] Each MVP action can write a history row
- [ ] User delete works
- [ ] Retention setting enforced on sweeper
- [ ] Clearing history does not delete Notes or Library items

## Privacy

History must not become a silent permanent screenshot archive. Align with [privacy.md](../architecture/privacy.md).

| Store | Intent | Retention |
|-------|--------|-----------|
| **Library** | Arrange & search captures/clips | Until user deletes |
| **Notes** | Structured documents / smart notes | Until user deletes |
| **History** | Recent action log | Short / configurable |

See [library.md](./library.md).
