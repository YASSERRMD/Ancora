# Streaming Chat Example

Demonstrates accumulating token text from a `MemoryStore` that holds
`ActivityRecorded` events in seq order, mirroring how a streaming agent
loop would produce output incrementally.

## What it tests

- Token text strings can be concatenated to form the full response
- `MemoryStore` preserves event insertion order (seq ascending)
- The last event is `RunCompleted`

## Pattern

```rust
use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_proto::ancora::journal_event::Event;

let store = MemoryStore::new();
// append started, token-a, token-b, completed events ...

let events = store.read("run-1").unwrap();
let text: String = events.iter()
    .filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).ok()?;
        v["text"].as_str().map(str::to_string)
    } else { None })
    .collect();

assert!(matches!(events.last().unwrap().event, Some(Event::RunCompleted(_))));
```

## Offline

All event types and `MemoryStore` are in-process.
