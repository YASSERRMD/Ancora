# Long-Horizon Agent Guide

ancora-lh provides primitives for agents that run for hours or days with
checkpoints, wakeups, progress persistence, signal injection, and throttling.

## Background Run Lifecycle

```
Created -> Running -> Sleeping -> Woken -> Running -> Completed
                                                    -> Failed
```

## Checkpointing

```rust
let mut cadence = CheckpointCadence::new(100);
if cadence.should_checkpoint(now) {
    let mut cp = Checkpoint::new(run_id, now);
    cp.set("cursor", &current_position.to_string());
}
```

## Scheduled and Event Wakeups

```rust
// Scheduled: fire at a specific tick
let w = ScheduledWakeup::new(run_id, 1000);
if w.should_fire(now) { run.wake(now); }

// Event-driven: fire when a named signal arrives
let mut ew = EventWakeup::new(run_id, "data-ready");
if ew.trigger("data-ready") { run.wake(now); }
```

## Idempotent Effects

Use `run.apply_effect(key)` to ensure effects are applied at most once
across wakeups, even if the run resumes from a checkpoint mid-step.

## Deadline Enforcement

```rust
let d = Deadline::new(run_id, deadline_tick);
d.check(now)?; // returns Err if past deadline
```

## Throttling

```rust
let mut throttle = Throttle::new(10); // 10 ops per tick
throttle.try_op(now)?;
```
