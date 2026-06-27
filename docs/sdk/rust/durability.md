# Durability (Rust)

## `JournalStore` -- persistent replay

Ancora records every activity to a journal so runs can resume after a crash.

```rust
use ancora_core::{Runtime, RuntimeOptions, SqliteStore, StoringTransport};

let store = SqliteStore::open("./agent_journal.db")?;
let rt = Runtime::with_options(RuntimeOptions {
    transport: Some(Box::new(StoringTransport::new(store))),
    ..Default::default()
})?;
```

## Resuming a run after restart

```rust
let run_id = "run_abc123"; // persisted from a prior session

let mut run = rt.resume(run_id).await?;
while let Some(ev) = run.next().await? {
    if let RunEvent::Completed { output } = ev {
        println!("{}", output);
    }
}
```

If the run completed before the restart, `resume` replays all recorded events
instantly without re-invoking the model.

## `MemoryStore` -- in-process (tests)

```rust
use ancora_core::MemoryStore;

let store = MemoryStore::new();
let rt = Runtime::with_options(RuntimeOptions {
    transport: Some(Box::new(StoringTransport::new(store))),
    ..Default::default()
})?;
```

`MemoryStore` keeps all events in RAM. Data is lost when the process exits.
Use it in tests to avoid file I/O.

## Custom `JournalStore`

Implement the `JournalStore` trait for any storage backend:

```rust
use ancora_core::JournalStore;

struct MyStore { /* ... */ }

#[async_trait::async_trait]
impl JournalStore for MyStore {
    async fn append(&self, run_id: &str, entry: &[u8]) -> anyhow::Result<()> { /* ... */ Ok(()) }
    async fn read(&self, run_id: &str) -> anyhow::Result<Vec<Vec<u8>>> { Ok(vec![]) }
}
```

## See also

- [Testing](testing.md)
- [Observability](observability.md)
