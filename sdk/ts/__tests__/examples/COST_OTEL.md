# cost-otel

Demonstrates wrapping an agent run in `Span` objects to record event counts,
total bytes, token estimates, and duration -- mirroring what an OTEL exporter
would collect. No OTEL SDK dependency required.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/cost-otel-example
```

## What it shows

- A minimal `Span` class with `setAttribute` and `end()` returning a `SpanRecord`
- The `estimateTokens` heuristic (4 chars per token)
- Attaching `run.id`, `event.count`, and `tokens.estimated` as span attributes
- Emitting a summary span after the run completes
