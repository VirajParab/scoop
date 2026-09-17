# Feature: Math engine

**Status:** Spec accepted  
**Priority:** P0

## Summary

Recognize mathematical expressions in the selection and compute results. Prefer a **local deterministic engine**; use AI only for ambiguous or word-problem cases.

## Examples

```text
₹1,25,000 × 12%
2^10
sqrt(144)
15% of 4,500
(12 × 45) / 3
```

Scenario B: `₹2,40,000 × 8.5% × 4` → `₹81,600`

## Output panel

```text
Expression:  <normalized>
Calculation: <steps if cheap to show>
Answer:      <value + currency symbol if applicable>
```

Actions: Copy answer, Copy expression, Save Note, Ask AI (explain).

## Parsing rules (MVP)

- Normalize `×` `·` `÷` to `*` `/`
- Handle `^` and `sqrt()`
- Percent: `X%` of `Y`; `Y × X%`
- Currency symbols preserved in display; strip for eval
- Indian grouping commas in numbers

## AI fallback

Use when:

- OCR text is a word problem
- Parse fails but content type is Math
- User clicks Solve / Explain

## Acceptance criteria

- [ ] Golden cases in [testing.md](../engineering/testing.md) pass
- [ ] Local path works offline
- [ ] Failure offers Ask AI without crashing
- [ ] Copy places answer on clipboard

## Non-goals (MVP)

- Full CAS / symbolic integration
- Matrix solvers
- Handwritten equation vision specialist
