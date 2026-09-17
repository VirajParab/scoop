# ADR-004: AI provider abstraction

**Status:** Accepted  
**Date:** 2026-09-17  
**Related:** [ai-providers.md](../architecture/ai-providers.md)

## Context

Product requires vision + text + structured outputs from multiple possible vendors, plus a future local path. Hard-coding one SDK would create vendor lock-in and block Free/Pro packaging flexibility.

## Decision

All model access goes through an **`AiProvider` interface** with capability flags. Concrete adapters (OpenAI, Anthropic, …) live behind the interface. UI and actions depend only on the interface.

Heuristic classification runs locally so the toolbar works when vision is slow or unavailable.

## Consequences

### Positive

- Swap providers in settings
- Test with mocks/fixtures
- Local models can implement the same trait (v3)

### Negative

- Lowest-common-denominator features unless capability-gated
- Per-provider prompt tuning still needed

### Follow-ups

- Ship OpenAI adapter first; Anthropic second
- Capability matrix in settings (“vision supported: yes/no”)

## Non-decision

Exact HTTP client and prompt templates — left to implementation, versioned with adapters.
