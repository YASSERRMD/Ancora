# cost_otel

Wraps an agent run in lightweight `Span` tracking to record event counts,
total bytes, token estimates, and duration -- mirroring what an OpenTelemetry
exporter would collect. No OTEL SDK dependency required.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.cost_otel
```

## What it shows

- Wrapping agent start and drain in timing spans
- Setting span attributes: `run.id`, `event.count`, `bytes.total`, `tokens.estimated`
- The 4-bytes-per-token estimation heuristic for usage without a usage header
- Emitting a summary span after the run completes
