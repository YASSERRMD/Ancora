# Durability Guide

## The journal is the source of truth

Every state transition in an Ancora run is written to the journal before the
agent proceeds. If the process is killed at any point, the journal contains
everything needed to resume from the last durable checkpoint.

## Crash recovery

Restart the process and call `run_graph` with the same `run_id`. The engine:

1. Reads all events for that `run_id` from the journal.
2. Calls `replay_events` to reconstruct the `ReplayState`.
3. Skips activities already in the replay state (their `activity_key` is
   recorded).
4. Continues execution from the earliest un-committed activity.

No user code changes are required for crash recovery. The guarantee is:
**exactly-once execution of side-effecting activities**.

## Exactly-once semantics

The engine appends an `ActivityRecordedEvent` to the journal only after the
activity's `execute()` function returns successfully. If the process crashes
before the event is appended, the activity runs again on the next start. If
it crashes after, the activity is skipped on replay.

This means:

- Activities that execute before the journal write may run more than once.
- Activities must be **idempotent** with respect to external systems (API
  calls, database writes, emails sent) to be safe under crashes.

If an activity is not naturally idempotent, implement a deduplication key in
the external system (an idempotency key in the API request, an `INSERT OR
IGNORE` in SQLite, etc.).

## Journal backends

| Backend | Durability | Use case |
|---------|-----------|----------|
| `MemoryStore` | None (process-lifetime) | Unit tests only |
| SQLite (`ancora-sqlite`) | WAL-mode, single-process | Local / embedded deployments |
| PostgreSQL | Full ACID with replication | Multi-process, high availability |

For production, use at least the SQLite backend with WAL mode enabled. For
multi-replica deployments, use PostgreSQL.

## Testing durability

Use the `FaultyStore` and `SeqFaultyStore` test helpers in
`crates/ancora-core/tests/chaos_store_failures.rs` to inject storage failures
at specific sequence numbers. The chaos tests in
`crates/ancora-core/tests/chaos_kill_resume.rs` simulate process kill and
resume scenarios.

Run all chaos tests:

```bash
cargo test -p ancora-core chaos
```

## Storage tuning

- **SQLite WAL mode**: enabled by default in `ancora-sqlite`. WAL allows
  concurrent reads during writes.
- **Checkpoint interval**: flush WAL to the main database file after every N
  writes (default: 1000). Lower values reduce recovery time; higher values
  improve write throughput.
- **Event batch size**: the journal write path is synchronous and single-event.
  For high-throughput workloads (many tool calls per second), batch events
  before committing.
