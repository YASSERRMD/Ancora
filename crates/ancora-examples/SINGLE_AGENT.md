# Single Agent Example

Demonstrates the `Run` lifecycle: `Pending -> Running -> Completed`, using
`MemoryStore` to persist journal events.

## What it tests

- `Run::generate()` produces a globally unique UUID-format ID
- A new run starts in `RunStatus::Pending`
- `run.transition(Running)` and `run.transition(Completed)` succeed in order
- `RunStatus::Completed` is terminal (`is_terminal() == true`)
- `MemoryStore::read` returns an empty slice for a new run

## Pattern

```rust
use ancora_core::run::{Run, RunStatus};

let mut run = Run::generate();
run.transition(RunStatus::Running).unwrap();
run.transition(RunStatus::Completed).unwrap();
assert!(run.status.is_terminal());
```

## Offline

No network calls. All types live in `ancora-core`.
