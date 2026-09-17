# ADR-002: OCR engine

**Status:** Proposed  
**Date:** 2026-09-17

## Context

MVP requires local English OCR for code, errors, prose, tables, and simple math — including dark themes — with editable output.

Candidates: Tesseract, PaddleOCR, OS/system OCR APIs.

## Decision

**Default to Tesseract** for MVP integration path, with a short bake-off against PaddleOCR on the fixture corpus in `testdata/ocr/`.

Interface OCR behind a trait so the engine can be swapped without rewriting actions.

```text
trait OcrEngine {
  fn recognize(&self, image: &RgbaImage) -> Result<OcrResult>;
}
```

## Consequences

### Positive

- Offline, privacy-friendly
- Mature packaging on Linux distros
- Swappable engine

### Negative

- Math/code accuracy may need preprocessing
- Language packs managed as dependency

### Follow-ups

- Score fixtures; switch default if PaddleOCR wins on code+dark UI
- Document `tesseract-ocr-eng` as runtime dependency

## Alternatives

| Engine | Notes |
|--------|-------|
| PaddleOCR | Strong accuracy; heavier / packaging cost |
| Cloud OCR | Rejected as default (privacy + latency) |
| OS OCR | Inconsistent across distros |
