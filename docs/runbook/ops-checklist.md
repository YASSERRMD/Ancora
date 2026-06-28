# Production Operations Checklist

## Daily checks

- [ ] No open P1 or P2 incidents
- [ ] Confirm backup ran successfully last night (check BackupOps logs)
- [ ] Review error rate Prometheus dashboard: `ancora_run_error_total` last 24h
- [ ] Check worker utilization: no worker > 90% for more than 5 minutes sustained

## Weekly checks

- [ ] Review open action items from recent post-mortems
- [ ] Confirm all scheduled migration locks have released
- [ ] Review cost chargeback report for anomalies
- [ ] Rotate secrets if `RotationLog::last_rotation_for` exceeds 30 days

## Before a deploy

- [ ] Confirm tests pass on main: `cargo test --workspace`
- [ ] Review migration plan: are zero-downtime patterns used?
- [ ] Enter maintenance window if schema changes require it
- [ ] Notify on-call secondary

## After a deploy

- [ ] Confirm `/healthz` and `/readyz` return OK on all workers
- [ ] Monitor error rate for 10 minutes
- [ ] Confirm queue depth returns to baseline
- [ ] Exit maintenance window

## Monthly

- [ ] Run load test baseline: `cargo test -p ancora-loadtest`
- [ ] Review SLO error budget: confirm budget remaining > 20%
- [ ] Review and update this checklist for accuracy
