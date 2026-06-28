# Long-horizon Agents

ancora-lh provides lifecycle management for background agents that run across
many ticks: scheduled and event wakeups, checkpointing, progress tracking,
signal injection, deadline enforcement, and per-tick throttling.

## Lifecycle

```rust
use ancora_lh::BackgroundRun;

let mut run = BackgroundRun::new("run-id", start_tick);
run.start();
run.apply_effect("fetched-data"); // idempotent: same string applied twice stays once
run.sleep_until(wake_tick);
// ... time passes ...
if run.wake(now_tick) {
    // back from sleep
}
run.complete();
```

## Wakeup Types

```rust
use ancora_lh::{ScheduledWakeup, EventWakeup};

let sched = ScheduledWakeup { run_id: "r1".into(), wake_at_tick: 100 };
sched.should_fire(now); // true when now >= 100

let mut event = EventWakeup::new("r1", "checkpoint_ready");
event.trigger("checkpoint_ready"); // fires once
```

## Checkpointing

```rust
use ancora_lh::{Checkpoint, CheckpointCadence};

let mut ck = Checkpoint::new("run-1", tick);
ck.set("phase", "step-3");

let mut cadence = CheckpointCadence::new(10); // every 10 ticks
if cadence.should_checkpoint(now_tick) {
    // save checkpoint
}
```

## Deadline and Throttle

```rust
use ancora_lh::{Deadline, Throttle};

let deadline = Deadline { run_id: "r".into(), deadline_tick: 200 };
deadline.check(now)?; // Err(DeadlineExceeded) if past deadline

let mut throttle = Throttle::new(5); // max 5 ops per tick
throttle.try_op(now)?; // Err(Throttled) if ops exhausted this tick
```
