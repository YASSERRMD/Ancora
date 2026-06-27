# Durability and Restart Recovery (Python)

Ancora records every activity to a journal so that a run can be resumed
after a crash without repeating completed steps.

## Enable durability

```python
from ancora import Runtime, SqliteStore, StoringTransport

store = SqliteStore("/var/lib/myapp/journal.db")
rt = Runtime(transport=StoringTransport(store))
```

With a `StoringTransport`, every run is journalled automatically. If the
process restarts mid-run, replay the journal to continue from the last
checkpoint:

```python
handle = rt.resume(run_id="run-abc-123")
result = handle.collect()
print(result.output)
```

## Assign a deterministic run ID

```python
result = rt.run(spec, "Summarise the report.", run_id="report-summary-2026-06-28")
```

Restarting with the same `run_id` replays completed activities from the
journal and re-runs only the remaining steps.

## In-memory store (tests)

```python
from ancora import MemoryStore, StoringTransport

rt = Runtime(transport=StoringTransport(MemoryStore()))
```

## Idempotency key templates

Tools that have side effects should declare an idempotency key template to
prevent double execution on replay:

```python
from ancora import ToolSpec

ToolSpec.from_callable(
    "send_email",
    send_email_fn,
    description="Send an email.",
    idempotency_key_template="send_email/{run_id}/{seq}",
)
```

## See also

- [Observability](observability.md)
- [Durability concept](../../concepts/durability-and-replay.md)
