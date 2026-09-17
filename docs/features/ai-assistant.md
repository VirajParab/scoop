# Feature: AI assistant (Ask AI)

**Status:** Spec accepted  
**Priority:** P0  
**Related:** [ai-providers.md](../architecture/ai-providers.md)

## Summary

Users ask questions about the selected content. Responses appear in a floating panel — no forced context switch to a separate chat app.

## Inputs to the model

- Screenshot (vision-capable models)
- OCR text (edited)
- Detected content type
- Optional application context (v2)
- User question

## Default prompts / entry points

| Entry | Suggested user-facing behavior |
|-------|--------------------------------|
| Ask AI | Empty question box + optional chips (“Explain”, “Debug”, “Summarize”) |
| Explain (code/math) | Prefills “Explain what this is and what I should do next.” |
| Debug | Prefills debugging-oriented prompt for errors |

## UI

```text
┌──────────────────────────────────────────┐
│ Ask AI                              [x]  │
├──────────────────────────────────────────┤
│ [editable OCR preview]                   │
│ Question: __________________________     │
│ [Send]                                   │
├──────────────────────────────────────────┤
│ Streaming response…                      │
├──────────────────────────────────────────┤
│ Copy │ Save Note │ Continue │ Close      │
└──────────────────────────────────────────┘
```

Stream tokens when the provider supports it.

## Errors

| Case | UX |
|------|----|
| No API key | CTA to Settings |
| Network error | Retry |
| Rate limit | Friendly wait / upgrade copy later |
| Cancel | Abort in-flight request |

## Acceptance criteria

- [ ] Works with screenshot + OCR context
- [ ] Copy / Save Note from response
- [ ] History entry created (respect retention)
- [ ] Dismiss cancels network call
- [ ] Offline: clear disabled state, not hang

## Privacy

Cloud send only on explicit Ask / Explain / related AI action. Indicator when network is used.
