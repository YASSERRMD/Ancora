# sqlite-persistence

Runs an agent with SQLite event persistence using `StoringTransport`.
After the run completes, retrieves and prints the stored events from the database.

## Run

```bash
go run ./examples/sqlite-persistence
# Creates example.db in the current directory
```

## What it shows

- Opening a `SqliteStore` with `OpenSqliteStore`
- Wrapping a `CgoTransport` with `NewStoringTransport`
- Persisting run IDs and events automatically during the run
- Retrieving stored events with `SqliteStore.EventsForRun`
- Checking total run count with `SqliteStore.RunCount`
