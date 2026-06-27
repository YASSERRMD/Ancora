# cost-otel

Demonstrates lightweight span tracking around an agent run, recording event
counts, duration, and an estimated token count -- mirroring the data that an
OpenTelemetry exporter would consume.

No OTEL SDK dependency is required; the spans are simple in-process structs
so the example stays fully offline.

## Run

```bash
cd sdk/go
go run ./examples/cost-otel
```

## What it shows

- Wrapping agent start and drain in timing spans
- Attaching `run.id`, `event.count`, and `tokens.estimated` attributes
- Printing a span summary on `end()`
- The 4-bytes-per-token estimation heuristic for usage without a usage header
