# Feature: Web search

**Status:** Spec accepted  
**Priority:** P0

## Summary

Search selected (OCR) content via a configurable web search provider in the user’s default or configured browser.

## Modes

### Exact Search

Use OCR / edited text as the query string.

### AI Search Query

AI rewrites the selection into a clearer query.

Example:

```text
Selected: "Postgres serialization failure"
Query:    "PostgreSQL serialization failure causes and solutions"
```

If AI unavailable → fall back to Exact Search with notice.

## Providers

Configurable:

- Google
- Bing
- DuckDuckGo

Construct URL templates; do not scrape SERP HTML in MVP.

## Flow

```text
Action: Search
  → Optional mode toggle (Exact | AI)
  → Build query
  → Open browser with provider URL
  → Record history (query only; no screenshot by default)
```

## Acceptance criteria

- [ ] Exact Search opens correct provider with encoded query
- [ ] AI mode requires provider config; degrades gracefully
- [ ] User can Copy the final query
- [ ] Setting persists across restarts

## Privacy

Query text may appear in browser history (outside Scoop). Document this. Scoop does not upload Exact Search queries to Scoop servers (none in MVP).
