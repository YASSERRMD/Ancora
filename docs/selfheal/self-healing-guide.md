# Ancora Self-Healing Guide

The `ancora-selfheal` crate provides the building blocks for making Ancora resilient in production.

## Liveness and Readiness Probes

```rust
use ancora_selfheal::{LivenessProbe, ReadinessProbe};

let mut liveness = LivenessProbe::new(30); // stall threshold: 30s
liveness.heartbeat(now);                    // called from the run loop
let status = liveness.check(now);          // ProbeStatus::Alive or Dead

let mut readiness = ReadinessProbe::new();
readiness.deps_healthy = journal.is_ok();
readiness.queue_saturated = queue.len() > HIGH_WATERMARK;
let status = readiness.check();            // ReadinessStatus::Ready or NotReady
```

Expose `liveness.check()` and `readiness.check()` on HTTP endpoints (`/healthz` and `/readyz`) for Kubernetes.

## Dependency Health Tracking

```rust
use ancora_selfheal::{DependencyHealth, DepStatus};

let mut health = DependencyHealth::new();
health.report("journal", DepStatus::Healthy);
health.report("vector-db", DepStatus::Degraded { reason: "high latency".into() });

if !health.is_all_healthy() {
    readiness.deps_healthy = false;
}
```

## Degraded Mode

```rust
use ancora_selfheal::DegradedController;

let mut ctrl = DegradedController::new();

// When streaming fails but everything else works
ctrl.enter_degraded(vec!["streaming".into()]);
assert!(ctrl.is_accepting_runs()); // still serving

// Full emergency: stop accepting new work
ctrl.enter_emergency();
assert!(!ctrl.is_accepting_runs());

ctrl.recover(); // back to normal
```

## Stuck-Run Detection and Auto-Requeue

```rust
use ancora_selfheal::{StuckRunDetector, AutoRequeue};

let mut detector = StuckRunDetector::new();
detector.register("run-abc", now, 300); // 5-minute timeout

// In the run loop:
detector.tick("run-abc", now);

// In the watchdog loop:
for run_id in detector.stuck_runs(now) {
    requeue.enqueue(run_id, now, backoff_secs);
    detector.remove(run_id);
}

let mut requeue = AutoRequeue::new(3); // max 3 attempts
let due = requeue.pop_due(now);
for entry in due {
    // re-dispatch entry.run_id
}
```

## Circuit Breakers

```rust
use ancora_selfheal::CircuitBreaker;

let mut cb = CircuitBreaker::new("openai", 5, 60);

if cb.is_open(now) {
    return Err(SelfHealError::CircuitOpen { name: "openai".into() });
}

match call_provider() {
    Ok(_) => cb.on_success(),
    Err(_) => cb.on_failure(now),
}

// After reset_timeout_secs, transition to half-open for a single probe
cb.try_half_open(now);
```

## Provider Automatic Failover

```rust
use ancora_selfheal::ProviderFailover;

let mut failover = ProviderFailover::new(vec!["openai".into(), "anthropic".into()]);

if let Some(provider) = failover.active_provider() {
    match call(provider) {
        Ok(r) => r,
        Err(_) => {
            failover.mark_failed(provider);
            let next = failover.failover().expect("no fallback provider");
            call(next)
        }
    }
}
```
