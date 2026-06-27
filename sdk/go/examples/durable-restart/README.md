# durable-restart

Demonstrates how to persist run events to a SQLite journal via
`StoringTransport` and replay them from the store after a simulated
process restart -- without re-running the agent.

Uses `:memory:` SQLite so no files are left on disk.

## Run

```bash
cd sdk/go
go run ./examples/durable-restart
```

## What it shows

- Opening a `SqliteStore` (in-memory for the example)
- Wrapping a `CgoTransport` with `StoringTransport`
- Persisting events automatically during `DrainEvents`
- Reading events back from the journal with `EventsForRun`
- Checking the total run count with `RunCount`
