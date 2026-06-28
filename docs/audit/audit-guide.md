# Audit Guide

The `ancora-audit` crate provides an immutable, tamper-evident audit log for all security-relevant events in Ancora.

## Core concepts

**AuditEntry** represents a single recorded event. Each entry carries a monotonic tick (not wall-clock time), a tenant identifier, a subject (user or service), an operation name, a resource path, an outcome, and a severity level. On creation the entry computes a checksum over its identity fields; mutating any field after creation invalidates the checksum and is detected by `verify()`.

**ImmutableAuditLog** is an append-only store. Every call to `append` assigns a monotonically increasing id and recomputes the checksum. Entries can never be updated or deleted through the public API, so the log is a true append-only record. `verify_all()` scans every entry for tampering.

**AuditEntryBuilder** provides a fluent API for constructing entries without exposing the raw constructor.

**AuditStats** aggregates totals, outcome counts, and severity counts over an iterator of entries, with a convenience `failure_rate()` accessor.

## Filtering

Use `ImmutableAuditLog` filter methods for point-in-time queries:

```rust
let t1_entries = log.filter_by_tenant("tenant-a");
let alice_entries = log.filter_by_subject("alice");
let reads = log.filter_by_operation("read");
let window = log.filter_by_tick_range(1000, 2000);
```

For multi-field queries use `AuditQuery`:

```rust
use ancora_audit::{AuditQuery, Outcome};

let results = AuditQuery::new()
    .tenant("tenant-a")
    .subject("alice")
    .outcome(Outcome::Failure)
    .tick_from(1000)
    .run(log.entries());
```

## Retention

`RetentionPolicy` identifies entries older than a tick-age threshold without deleting them (the log is immutable). Use the returned ids to decide whether to archive or alert.

```rust
use ancora_audit::RetentionPolicy;
let policy = RetentionPolicy::new(10_000);
let stale_ids = policy.evict(&log, current_tick);
```

## Export

`to_json` and `to_csv` convert a slice of entry references to string output for reporting or archiving.

## Multi-tenant summary

`summarize_by_tenant` groups entries by tenant and returns per-tenant `AuditStats`, sorted by tenant id.
