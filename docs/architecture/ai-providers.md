# AI provider architecture

**Status:** Accepted  
**Related ADR:** [ADR-004](../decisions/ADR-004-ai-abstraction.md)

## Principle

Do **not** hard-code Scoop around one AI vendor. All model calls go through a provider interface so product and business stay flexible.

```text
AI Provider Interface
        ↓
 ┌──────┼─────────┐
OpenAI  Local     Other
        Models*
```

\* Local models: v3.

## Capabilities

Providers may implement a subset of capabilities:

| Capability | Used by |
|------------|---------|
| `classify_vision` | Content type for toolbar |
| `complete_text` | Ask AI, smart notes, AI search query |
| `complete_vision` | Ask AI with screenshot, UI understand |
| `structured_output` | Smart notes JSON, classifications |

## Interface (conceptual)

```rust
// Pseudocode — language follows chosen stack

enum ContentType {
  Text, Code, Math, Table, Chart, Image,
  UiScreenshot, Product, Document, Unknown,
}

struct SelectionContext {
  image_path: PathBuf,
  ocr_text: String,
  content_type: Option<ContentType>,
  app_context: Option<AppContext>, // v2
}

trait AiProvider {
  fn id(&self) -> &str;
  fn capabilities(&self) -> CapSet;

  async fn classify(&self, ctx: &SelectionContext) -> Result<ContentType>;
  async fn ask(&self, ctx: &SelectionContext, question: &str) -> Result<String>;
  async fn rewrite_search_query(&self, text: &str) -> Result<String>;
  async fn structure_note(&self, ctx: &SelectionContext) -> Result<SmartNoteDraft>;
  // v2:
  async fn generate_html(&self, ctx: &SelectionContext, opts: HtmlOpts) -> Result<HtmlBundle>;
}
```

## Configuration

Per settings:

- Provider id (`openai`, `anthropic`, `google`, …)
- API key (secret store)
- Model name(s) for text vs vision
- Temperature (where applicable)
- Cloud processing enabled/disabled

If cloud disabled and no local provider: actions requiring AI show a clear setup prompt.

## Fallback behavior

| Situation | Behavior |
|-----------|----------|
| No API key | Disable AI actions; keep OCR, Math, Exact Search, Notes, Copy |
| Vision fails | Fall back to OCR + heuristic content type |
| Classify timeout | Show universal actions; upgrade toolbar when classify returns |
| Rate limit / quota | Surface actionable error; don’t crash overlay |

## Heuristic classify (no vision)

Before/without vision, use local heuristics on OCR text:

- Math tokens (`√`, `%`, `^`, operators, currency)
- Code cues (braces, `error:`, stack traces, indentation)
- URLs / prose → Text / Document

Vision refines when available.

## Cost control

- Prefer local math over AI for deterministic expressions
- Prefer exact search unless user chooses AI query mode
- Avoid re-sending large images when OCR-only suffices
- Log token/image usage when telemetry enabled (opt-in)

## Provider roadmap

| Provider | MVP | Later |
|----------|-----|-------|
| OpenAI | Yes | — |
| Anthropic | Yes if capacity | — |
| Google | Stretch | Yes |
| Local (Ollama / etc.) | — | v3 |
