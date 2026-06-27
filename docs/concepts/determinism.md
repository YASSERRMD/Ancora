# Determinism

Ancora makes agent runs **deterministic with respect to the journal**. Given
the same sequence of journal events, two runs of the same code always produce
the same output.

## Why determinism matters

- **Debugging**: reproduce a bug by replaying its journal.
- **Testing**: assert on a fixed event sequence without a live model.
- **Audit**: prove what the agent did by reading the journal.

## Sources of non-determinism

Ancora controls these sources:

| Source | How Ancora handles it |
|--------|----------------------|
| Model sampling temperature | Cached in `ActivityRecorded` on first call |
| Tool side effects | Cached before execution; replayed without re-running |
| Timestamps | Stored in journal events; replay uses stored values |
| Random IDs | Run IDs generated once at `RunStarted`; stable on replay |

## What is NOT deterministic

- The **first** call to a non-deterministic model (before the result is
  cached in the journal). Ancora cannot control model sampling.
- External state changed between runs (database rows updated, files moved).

## Idempotency keys

Activity keys uniquely identify each tool call within a run. The key
template `{run_id}-{node_id}-{seq}` ensures that no two activities for
the same call share a key, and that replay matches the correct cached result.

## See also

- [Durability and Replay](durability-and-replay.md)
- [Tools and Effects](tools-and-effects.md)
