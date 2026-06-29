# Conflict Resolution

A conflict occurs when two or more devices write different values to the same
key during an offline period. ancora-sync detects and resolves these conflicts
using a configurable policy.

## Detection

`ConflictDetector::detect` scans a flat list of `JournalEntry` values for pairs
that share the same `key` but differ in `device_id` or `payload`. Each pair
produces a `Conflict` struct.

## Resolution Policies

| Policy | Behaviour |
|--------|-----------|
| `LastWriteWins` | The entry with the higher sequence number wins |
| `PreferLocal` | The local (device-originated) entry always wins |
| `PreferRemote` | The remote (hub-received) entry always wins |
| `Merge` | Both payloads are concatenated to form a new entry |

### Choosing a Policy

* **LastWriteWins** is appropriate for configuration keys where the most recent
  write is authoritative.
* **PreferLocal** is suitable when edge-device autonomy is paramount (e.g.
  sensor readings that should never be overwritten by a stale hub value).
* **PreferRemote** suits scenarios where a central operator should always
  override device-local state.
* **Merge** works well for append-only data structures (e.g. event logs, CRDT
  sets) where all writes are additive.

## API

```rust
use ancora_sync::conflict::{ConflictDetector, ConflictPolicy};

let conflicts = ConflictDetector::detect(&entries);
let resolved  = ConflictPolicy::LastWriteWins.resolve_all(&conflicts);
```
