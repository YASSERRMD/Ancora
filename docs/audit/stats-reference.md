# AuditStats Reference

`AuditStats` is computed from an iterator of `AuditEntry` references and aggregates the following fields.

## Fields

| Field | Type | Description |
|---|---|---|
| `total` | `usize` | Total number of entries in the sample |
| `successes` | `usize` | Entries with `Outcome::Success` |
| `failures` | `usize` | Entries with `Outcome::Failure` |
| `blocked` | `usize` | Entries with `Outcome::Blocked` |
| `critical` | `usize` | Entries with `Severity::Critical` |
| `errors` | `usize` | Entries with `Severity::Error` |

## Derived metrics

`failure_rate()` returns `failures as f64 / total as f64`, or `0.0` when `total == 0`.

## Outcome values

| Variant | Meaning |
|---|---|
| `Success` | The operation completed as requested |
| `Failure` | The operation was attempted but did not complete |
| `Blocked` | The operation was rejected before execution (policy deny, auth failure) |

## Severity values

| Variant | Meaning |
|---|---|
| `Info` | Normal operational event |
| `Warning` | Unusual but non-critical condition |
| `Error` | Operational error that may require attention |
| `Critical` | Severe event requiring immediate attention |

## Usage example

```rust
use ancora_audit::{AuditStats, ImmutableAuditLog};

let stats = AuditStats::from_entries(log.entries());
println!("failure rate: {:.1}%", stats.failure_rate() * 100.0);
println!("critical events: {}", stats.critical);
```
