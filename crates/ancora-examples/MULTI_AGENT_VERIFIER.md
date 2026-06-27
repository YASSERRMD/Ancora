# Multi-Agent Verifier Example

Demonstrates running a primary agent and a verifier agent concurrently using
`std::thread::spawn`, verifying that each run receives a distinct ID.

## What it tests

- Two `Run::generate()` calls produce distinct IDs
- Multiple threads can generate run IDs concurrently without collision
- Concurrent runs are independent (separate `Run` structs)

## Pattern

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use ancora_core::run::Run;

let ids: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

let handles: Vec<_> = (0..4).map(|_| {
    let ids = Arc::clone(&ids);
    thread::spawn(move || {
        let run = Run::generate();
        ids.lock().unwrap().push(run.id);
    })
}).collect();

for h in handles { h.join().unwrap(); }

let ids = ids.lock().unwrap();
let unique: std::collections::HashSet<&String> = ids.iter().collect();
assert_eq!(ids.len(), unique.len());
```

## Offline

No network calls. `Run::generate()` uses `uuid::Uuid::new_v4()`.
