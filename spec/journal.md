# Ancora Journal Format and Replay Contract

Source of truth: `crates/ancora-proto/proto/journal.proto`

## Purpose

The event journal is the foundation of Ancora's durability guarantees.
Every non-deterministic activity (model call, tool call, human decision)
is recorded in the journal before its result is used. On crash recovery
or explicit replay, the engine re-executes the same code path but
returns journaled results instead of re-invoking the activities.

## JournalEvent envelope

```proto
message JournalEvent {
  string event_id       = 1;  // globally unique (UUIDv7 or ULID)
  string run_id         = 2;  // stable run identifier
  uint64 seq            = 3;  // monotonically increasing within the run
  int64  recorded_at_ns = 4;  // Unix epoch, nanoseconds
  oneof  event { ... }        // exactly one variant (fields 10-19)
}
```

`seq` starts at 0 for the `RunStarted` event of each run and increments
by 1 for every subsequent event. No two events in the same run share
a `seq` value. The replay engine uses `seq` to order events when
loading from storage.

## Event variants

| Field | Type | When emitted |
|-------|------|-------------|
| `run_started` | `RunStartedEvent` | Before any work begins |
| `node_entered` | `NodeEnteredEvent` | When execution enters a graph node |
| `node_exited` | `NodeExitedEvent` | When execution exits a graph node |
| `activity_recorded` | `ActivityRecordedEvent` | After each model/tool call |
| `human_decision_requested` | `HumanDecisionRequestedEvent` | When the run suspends for human input |
| `human_decision_received` | `HumanDecisionReceivedEvent` | When the human responds |
| `run_completed` | `RunCompletedEvent` | On successful final output |
| `error` | `ErrorEvent` | On any error |
| `retry_scheduled` | `RetryScheduledEvent` | Before each retry delay |
| `run_cancelled` | `RunCancelledEvent` | When the run is cancelled |

## ActivityRecordedEvent

```proto
message ActivityRecordedEvent {
  string activity_key  = 1;  // stable idempotency key
  string activity_kind = 2;  // "model_call" | "tool_call" | ...
  string input_json    = 3;  // JSON-encoded input
  string result_json   = 4;  // JSON-encoded result
  bool   replayed      = 5;  // true when returned from journal, not live
}
```

The `activity_key` is derived from `{run_id}-{node_id}-{seq}` and is
stable across restarts. On replay, when the engine encounters the same
key in the journal it sets `replayed = true` and returns the stored
`result_json` without calling the underlying activity.

## Replay contract

1. The journal is append-only. No event is modified or deleted.
2. On replay, the engine reads events in `seq` order and reconstructs
   state by folding them.
3. If the code path produces a different event type at a given `seq`
   than what is in the journal, the engine raises a
   `NondeterminismError` and halts.
4. `recorded_at_ns` is informational and does not affect replay logic.
   Replay is not time-dependent.
5. A `replayed = true` flag on `ActivityRecordedEvent` is informational
   for observability. It does not change the replay result.

## Storage requirements

- Events must be stored durably before being considered journaled.
- The idempotency index on `activity_key` must reject duplicate writes.
- Reads must return events in ascending `seq` order for a given `run_id`.
