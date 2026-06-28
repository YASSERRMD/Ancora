# Deadlock and Conflict Handling

## Deadlock Prevention

Assign a strict total order to all shared resources and always acquire them
in that order. With a total order, cycles cannot form.

## Deadlock Detection

```rust
let mut det = DeadlockDetector::default();
det.add_wait("agent-a", "agent-b");
det.add_wait("agent-b", "agent-a");
if det.has_deadlock() {
    let victim = det.break_cycle()?;
    // abort victim's current operation and requeue it
}
```

## Conflict Resolution

When two agents race for the same resource, use `ConflictResolver::resolve`:

- `HighestPriority`: for quality-sensitive resources (best agent wins)
- `FirstCome`: for fairness under equal capability
- `Random(seed)`: for load balancing (deterministic with a fixed seed)

## Journaling

Record every deadlock detection and conflict resolution event in `CoordJournal`
so the resolution can be replayed deterministically in tests and audits.
