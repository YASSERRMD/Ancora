# Disaster Recovery Runbook

## Roles

| Role | Description |
|------|-------------|
| Primary | Accepts all writes; source of truth |
| Secondary | Continuously replicated replica; promotes on failover |
| Standby | Old primary after failover; fenced; no writes |

## Failover procedure

1. Verify replication lag: `replication_lag(&primary, &secondary)`. Must be <= RPO threshold.
2. Fence the primary: `primary.fence()` - all subsequent write attempts return an error.
3. Promote secondary: `secondary.unfence()`.
4. Update routing to point clients at the secondary.
5. Enumerate in-flight runs via `RunTracker::runs_to_resume()` and re-queue them.

Using the DR controller:
```rust
use ancora_dr::FailoverController;

let mut ctrl = FailoverController::new();
ctrl.failover(&mut primary, &mut secondary, max_lag_entries)?;
```

## Failback procedure

After the original primary is repaired:
```rust
ctrl.failback(&mut old_primary, &mut current_primary)?;
```

1. Unbar old primary from writes.
2. Sync old primary with all entries written during failover.
3. Fence current primary.
4. Restore routing to old primary.

## Automated DR drill

Run a full drill in-memory to validate failover and failback paths:
```rust
use ancora_dr::drill::run_drill;

let result = run_drill(&mut primary, &mut secondary, rto_secs, max_lag);
assert!(result.passed);
```
