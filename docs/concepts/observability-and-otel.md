# Observability and OTEL

Ancora exports traces and cost data via the OpenTelemetry (OTEL) protocol.
Every agent run produces spans that cover model calls, tool dispatches, and
the overall run lifecycle.

## Span hierarchy

```
agent.run  [root span]
  model.complete  [one per model call]
  tool.dispatch   [one per tool call]
    <tool-name>   [user-defined child]
```

## Exported attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `ancora.run_id` | string | Stable run identifier |
| `ancora.model_id` | string | Model used for the call |
| `ancora.input_tokens` | int | Prompt token count |
| `ancora.output_tokens` | int | Completion token count |
| `ancora.cost_usd` | float | Estimated cost in USD |
| `ancora.tool_name` | string | Name of the dispatched tool |

## Cost tracking

Token usage is summed per run and exposed via `RunHandle.getCost()` (all
SDKs). The cost estimate uses provider-specific pricing tables shipped with
`ancora-inference`.

## In-process spans

For development and offline testing, `ancora-observability` provides an
in-process span builder that does not require an OTEL collector:

```rust
use ancora_examples::Span;

let mut s = Span::new("agent.run");
s.set_attribute("run.id", run.id.as_str());
let ms = s.end_ms();
```

## OTEL exporter configuration

Set `ANCORA_OTEL_ENDPOINT` to an OTLP HTTP or gRPC endpoint:

```
ANCORA_OTEL_ENDPOINT=http://localhost:4317
```

## See also

- [Observability guide](../guides/observability.md)
- [Cost and OTEL example](../sdk/rust/index.md)
