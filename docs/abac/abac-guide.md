# ABAC Guide

Attribute-based access control evaluates policies against subject, resource, and environment
attributes at decision time.

## Core concepts

- **Subject attributes**: properties of the principal making the request (role, department, clearance)
- **Resource attributes**: properties of the resource being accessed (classification, owner, tags)
- **Environment attributes**: context at request time (time, region, request source)

## Quick start

```rust
use ancora_abac::*;

let mut store = PolicyStore::new();
store.add(Policy::new(
    "allow-eng-read",
    Effect::Allow,
    vec!["read".into()],
    Condition::Eq("dept".into(), AttributeValue::String("engineering".into())),
));

let ctx = RequestContext::new()
    .with_subject_attr("dept", "engineering");

let engine = PolicyEngine::new(&store);
let decision = engine.evaluate("read", &ctx.subject, &ctx.resource, &ctx.environment);
```

## Condition types

| Condition | Description |
|---|---|
| `Eq(key, value)` | exact equality |
| `NotEq(key, value)` | inequality |
| `GreaterThan(key, n)` | integer comparison |
| `LessThan(key, n)` | integer comparison |
| `Contains(key, val)` | list membership |
| `Exists(key)` | attribute presence |
| `And / Or / Not` | logical composition |

## Policy evaluation order

Policies are evaluated in ascending priority order (lower number = first).
The first matching `Deny` wins over any `Allow`.
If no policy matches, the result is `NotApplicable`.
