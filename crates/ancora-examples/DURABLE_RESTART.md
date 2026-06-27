# Durable Restart Example

Demonstrates persisting run events to `MemoryStore` (and the lightweight
`RunJournal` wrapper) and replaying them after a simulated process restart.

## What it tests

- `RunJournal.record_run` is idempotent
- `RunJournal.events_for_run` returns an empty slice for unknown runs
- `MemoryStore` persists events across multiple `read` calls
- Multiple independent runs are stored separately

## Pattern

```rust
use ancora_examples::RunJournal;
use ancora_core::journal::{JournalStore, MemoryStore};

let mut journal = RunJournal::new();
journal.record_run("run-1");
journal.append_event("run-1", r#"{"kind":"started"}"#);

let store = MemoryStore::new();
store.append("run-1", start_event("run-1")).unwrap();
store.append("run-1", complete_event("run-1")).unwrap();

// Simulate restart: replay from store
let replayed = store.read("run-1").unwrap();
assert_eq!(2, replayed.len());
```

## Offline

All types are in-process. No persistence to disk.
