# Common Day-Two Procedures

## Drain and restart a worker

1. Mark worker as draining: `registry.drain("worker-1")`
2. Poll until `is_drained("worker-1")` returns true.
3. Restart the pod: `kubectl rollout restart deployment/ancora-worker`

## Cancel a stuck run

```
store.cancel("run-<id>")
```

If the run has already completed, cancel returns false (safe to ignore).

## Emergency tenant suspend

```
ops.suspend("tenant-id")
```

Suspension prevents new runs but does not cancel in-flight runs. Cancel those separately.

## Manual backup

```rust
let bkp = backup_ops.create_backup("tenant-id", now_secs);
// Persist bkp to external storage
```

## Failover

1. Drain primary: `registry.drain("primary-worker")`
2. Wait for drain to complete.
3. Execute failover: trigger via DR controller (`FailoverController::failover`).
4. Confirm secondary is promoting runs correctly.
5. Monitor `replication_lag` on new primary.

## Rollback after a bad deploy

1. `BlueGreenController::rollback()` immediately restores the previous blue pool.
2. Monitor `encore_run_failure_total` metric for the next 5 minutes.
3. If stable, investigate green pool failure before re-promoting.

## Config dump (redacted)

```rust
use ancora_config::redacted_dump;
let dump = redacted_dump(&cfg);
println!("{}", serde_json::to_string_pretty(&dump).unwrap());
```
