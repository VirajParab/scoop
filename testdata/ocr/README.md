# OCR fixtures

Place PNG samples here for regression tests:

| File | Expectation |
|------|-------------|
| `code_dark_terminal.png` | Error lines readable |
| `math_currency.png` | Expression OCR + math parse |
| `serif_pdf_light.png` | Prose paragraph |
| `table_simple.png` | Cell text roughly extracted |

Generate fixtures after installing `tesseract-ocr`. Unit tests for math live in Rust (`math_engine` tests) and do not require images.
