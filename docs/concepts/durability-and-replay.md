# Durability and Replay

Ancora makes every agent run **durable by default**. If the host process
crashes mid-run, the run resumes from the last safe checkpoint without data
loss or repeated side effects.

## How it works

1. Before executing any side-effecting tool, Ancora writes an
   `ActivityRecorded` event to the journal with a unique activity key.
2. The tool executes and its result is written to the same event.
3. On replay, if Ancora encounters an activity key that already exists in the
   journal, it returns the cached result without re-executing the tool.

This guarantees that `WRITE` effects happen **at most once** even across
crashes and restarts.

## Journal stores

| Store | Use case |
|-------|----------|
| `MemoryStore` | Tests and single-process runs |
| `SqliteStore` | Single-binary edge deployment |
| `PostgresStore` | Multi-replica production deployment |

## Checkpoints

At the end of each graph node, Ancora saves a checkpoint blob. On resume,
replay starts from the most recent checkpoint rather than replaying the
entire journal from the beginning.

## Replay example

```
Run crashes after tool call A but before tool call B.

Journal at crash:
  seq=0  RunStarted
  seq=1  ActivityRecorded {key="run-1-node-1-0", result="..."}

On restart:
  seq=0  RunStarted    -> initialise run
  seq=1  ActivityRecorded -> return cached result for A (no re-execution)
  seq=2  ActivityRecorded -> execute B (new)
  seq=3  RunCompleted
```

## See also

- [Determinism](determinism.md)
- [Durability guide](../guides/durability.md)
