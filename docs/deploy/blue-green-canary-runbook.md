# Blue-Green and Canary Deploy Runbook

## Blue-green switch

1. Prepare the green pool (new version workers, all idle).
2. Drain the blue pool: wait for `active_runs == 0` on all blue workers.
3. Switch: `ctrl.switch()`. Traffic shifts to green instantly.
4. Verify green health via monitoring dashboards.
5. If issues arise: `ctrl.rollback()` restores blue.

```rust
use ancora_deploy::{BlueGreenController, VersionedWorker, Version};

let mut ctrl = BlueGreenController::new(blue_workers, green_workers);
ctrl.switch()?; // fails with DrainIncomplete if blue is still active
```

## Canary rollout

1. Register a small canary pool (new version) alongside the stable pool.
2. Set canary traffic percentage (e.g., 10%).
3. Monitor the canary health gate.
4. Promote if healthy; rollback if gate fails.

```rust
use ancora_deploy::CanaryController;

let mut ctrl = CanaryController::new(stable, canary, 0.10, 5.0);
// For each request:
if ctrl.route_to_canary(request_index) {
    // route to canary
    ctrl.record_canary_result(is_error);
    ctrl.check_health_gate()?; // returns Err if error rate > 5%
}
// All healthy:
ctrl.promote();
```

## Mixed-version interoperability

Workers with the same major version are considered compatible.
Workers with different major versions must not share a journal store:
`assert_compatible(&worker_a.version, &worker_b.version)?`.

## Deploy history

Every switch, rollback, canary start, and promotion should be recorded:
```rust
use ancora_deploy::{DeployHistory, DeployEvent};

let mut hist = DeployHistory::new();
hist.record(DeployEvent::BlueGreenSwitch { from, to, duration_ms });
```
