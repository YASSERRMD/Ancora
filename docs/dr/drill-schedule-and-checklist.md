# DR Drill Schedule and Checklist

## Schedule

| Drill type | Frequency |
|-----------|-----------|
| Automated in-memory drill | Every CI run |
| Staging environment failover | Monthly |
| Production failover simulation | Quarterly |

## Pre-drill checklist

- [ ] Verify secondary is within RPO lag threshold
- [ ] Notify stakeholders of planned maintenance window
- [ ] Confirm backup of all journal data
- [ ] Ensure monitoring dashboards are accessible

## Drill execution

Run the automated drill:
```bash
cargo test -p ancora-dr -- test_drill
```

For a staging drill:
1. Induce a primary outage (stop the primary process).
2. Wait for automatic failover to trigger (or trigger manually via `FailoverController::failover`).
3. Verify clients reconnect and resume work.
4. Measure time from outage to service restoration.
5. Run `failback` to restore the primary.

## Post-drill checklist

- [ ] Record failover time and compare to RTO
- [ ] Verify zero duplicate run executions
- [ ] Confirm all in-flight runs resumed on new primary
- [ ] Update DR runbook if any steps failed
- [ ] File a ticket for any RTO/RPO violations

## Failure criteria

A drill fails if:
- Failover time exceeds RTO
- Any data loss exceeds RPO
- A run executes twice (duplicate side effect)
- The old primary accepts writes after fencing
