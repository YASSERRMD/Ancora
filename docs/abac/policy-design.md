# ABAC Policy Design

## Deny-by-default pattern

To implement deny-by-default, add a single low-priority allow policy and
let the absence of a matching allow result in `NotApplicable`.
Treat `NotApplicable` as `Deny` at the API boundary.

```rust
fn is_authorized(decision: &Decision) -> bool {
    matches!(decision, Decision::Allow)
}
```

## Priority ordering

Lower `priority` numbers are evaluated first. Use this to implement
override-deny policies:

```rust
// Evaluated first (priority=1) -- blocks all blocked users unconditionally
store.add(deny_if_blocked().with_priority(1));
// Evaluated second (priority=50) -- allows engineering to read
store.add(allow_if_department("engineering").with_priority(50));
```

## Composing conditions

Use `And`, `Or`, and `Not` to build complex conditions:

```rust
Condition::And(
    Box::new(Condition::Eq("dept".into(), "security".into())),
    Box::new(Condition::LessThan("clearance_level".into(), 3)),
)
```

## Resource attribute patterns

Use resource attributes for data sensitivity enforcement:

```rust
// Only allow reads when the resource classification level <= subject clearance
Condition::And(
    Box::new(Condition::Exists("subject_clearance".into())),
    Box::new(Condition::LessThan("resource_classification".into(), 3)),
)
```
