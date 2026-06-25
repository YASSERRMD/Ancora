# Observability Guide

## What Ancora emits

Each run produces a stream of `JournalEvent` messages. These are the primary
observability surface:

| Event | When emitted |
|-------|-------------|
| `RunStartedEvent` | At the start of a new run |
| `NodeEnteredEvent` | When execution enters a node |
| `NodeExitedEvent` | When a node finishes (success or failure) |
| `ActivityRecordedEvent` | After an activity completes and is durably stored |
| `RunCompletedEvent` | When all nodes have finished |

## Structured logging

Export journal events to your logging pipeline by reading the journal after
the run:

```rust
let events = store.read(run_id)?;
for ev in &events {
    tracing::info!(
        run_id = %ev.run_id,
        seq = ev.seq,
        kind = event_kind(&ev),
        "journal event"
    );
}
```

## OpenTelemetry spans

Ancora emits OpenTelemetry GenAI spans when the `otel` feature is enabled.
Each `NodeEnteredEvent`/`NodeExitedEvent` pair produces a child span under
the run's root span. Span attributes include:

- `gen_ai.system` -- the model provider
- `gen_ai.request.model` -- the model ID
- `gen_ai.usage.input_tokens` -- tokens consumed
- `gen_ai.usage.output_tokens` -- tokens produced

Configure the OTLP exporter:

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 cargo run
```

## Cost tracking

Each agent node records token usage in its `ActivityRecordedEvent`. Sum the
usage fields across a run for a per-run cost estimate:

```rust
let total_tokens: u64 = events.iter()
    .filter_map(|ev| match &ev.event {
        Some(Event::ActivityRecorded(a)) => {
            let v: serde_json::Value = serde_json::from_str(&a.result_json).ok()?;
            v["usage"]["total_tokens"].as_u64()
        }
        _ => None,
    })
    .sum();
```

## Masking sensitive content for cross-language comparison

The `journal_mask` module strips model-generated content before journal export
or comparison:

```rust
use ancora_core::journal_mask::{mask_events, assert_structurally_equal};

let masked = mask_events(&events);
assert_structurally_equal(&masked_a, &masked_b)?;
```

Use this for cross-language conformance testing and for logging pipelines that
must not retain model outputs (PII, confidential data).

## Distributed tracing

For multi-agent runs involving A2A handoffs, propagate the W3C `traceparent`
header through handoff requests. Each remote agent creates a child span under
the originating trace context. This gives end-to-end visibility across agent
boundaries in tools like Jaeger, Grafana Tempo, or Honeycomb.
