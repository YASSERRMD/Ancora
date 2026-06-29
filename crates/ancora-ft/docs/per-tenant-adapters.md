# Per-Tenant Adapter Selection

`ancora-ft` provides first-class support for selecting different adapters
on a per-tenant basis, with a journaled event log that supports full replay.

## Concept

In a multi-tenant deployment, different tenants may require different
fine-tuned adapters -- for example:

- Tenant A (legal): legal-reasoning LoRA
- Tenant B (support): customer-support LoRA
- Tenant C (code): code-generation LoRA

The `TenantAdapterMap` stores the current active mapping. The
`SelectionJournal` provides an append-only audit trail of every change.

## Usage

```rust
use ancora_ft::SelectionJournal;
use ancora_ft::journal::select_for_tenant;
use ancora_ft::runtime::TenantAdapterMap;
use ancora_ft::AdapterId;

let mut journal = SelectionJournal::new();
let mut map = TenantAdapterMap::new();

// Assign adapters to tenants.
select_for_tenant(&mut journal, &mut map, "tenant-legal", AdapterId::new("legal-v1"));
select_for_tenant(&mut journal, &mut map, "tenant-support", AdapterId::new("support-v2"));

// Query current assignment.
let id = map.get("tenant-legal").unwrap();
println!("legal tenant uses: {}", id);
```

## Replay

The journal can reconstruct the full `TenantAdapterMap` from scratch:

```rust
// Reconstruct state from journal (e.g., after restart).
let reconstructed = journal.replay();
let id = reconstructed.get("tenant-legal").unwrap();
```

This makes the selection state durable: persist `journal.events()` to disk
or a database, and replay on startup to restore state without a separate
snapshot.

## Clearing Assignments

```rust
use ancora_ft::journal::clear_for_tenant;

clear_for_tenant(&mut journal, &mut map, "tenant-legal");
// Replay will reflect the cleared assignment.
```

## Audit Queries

```rust
// All events for a specific tenant.
let events = journal.events_for_tenant("tenant-legal");
for e in events {
    println!("seq={} adapter={:?}", e.seq, e.adapter_id);
}
```

## Thread Safety

`TenantAdapterMap` and `SelectionJournal` are not `Sync`. In a concurrent
context, wrap them in `Arc<Mutex<_>>` or use a channel-based actor pattern.
