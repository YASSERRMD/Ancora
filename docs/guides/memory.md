# Memory and State Guide

## Within-run memory

Each agent node in a run has access to:

- **Short-term context**: the model's own context window (conversation history
  and tool call results within the current node's execution).
- **Run output**: the output JSON of any previously completed node in the same
  run, accessible via the graph execution context.

There is no implicit cross-node shared memory. Nodes communicate by returning
structured output that downstream nodes receive as input.

## Cross-run memory

Ancora does not implement a vector store or retrieval system. Cross-run memory
is implemented at the tool level:

1. Write a `MemoryWriteTool` that persists data to a database or file.
2. Write a `MemoryReadTool` that retrieves data from the same store.
3. Register both tools on the agent that needs cross-run context.

This keeps the core engine simple and lets you choose the right storage backend
(SQLite, PostgreSQL, a vector DB) per use case.

## Journal as audit memory

The journal is a persistent, append-only record of every event in every run.
It is the authoritative source for:

- Replay (crash recovery, idempotency).
- Debugging (what did the agent do and why).
- Compliance (what actions were taken, and when).

Query the journal directly for retrospective analysis:

```rust
let events = store.read("run-abc")?;
for ev in &events {
    println!("{}: {:?}", ev.seq, ev.event);
}
```

## Replay state

On restart the engine reconstructs `ReplayState` from the journal. The replay
state records which activities have already been executed (keyed by
`activity_key`). Activities already in the replay state are skipped; only
pending activities are re-executed.

This guarantees **exactly-once execution** of side-effecting activities even
across crashes. See the [Durability guide](./durability.md) for details.

## Recommended patterns

- **Stateless tools**: keep tools stateless; read and write explicit storage
  tools rather than mutating hidden state.
- **Idempotent activities**: design activities so that re-running them (before
  the journal commit) is safe. The engine protects against double-execution
  post-commit but not against non-idempotent pre-commit failures.
- **Key your memory**: use deterministic, human-readable `activity_key` values
  (`"fetch:user:42"`, `"send:email:order:789"`) so the journal is readable as
  an audit log.
