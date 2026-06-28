# Alert Runbooks

## HighErrorRate

**Severity:** Critical

**Condition:** Run error rate > 5% over 5 minutes.

**Investigation:**
1. Check recent deployments (`kubectl rollout history`).
2. Review provider error logs for API failures.
3. Check `ancora_provider_error_rate` metric per provider.

**Resolution:**
- If provider issue: switch to backup provider or reduce concurrency.
- If code bug: roll back the deployment.
- If quota exceeded: review tenant quota settings.

## QueueBacklog

**Severity:** Warning

**Condition:** Queue depth > 100 pending runs.

**Investigation:**
1. Check worker count: `kubectl get pods -l app=ancora-worker`.
2. Check `ancora_worker_utilization` metric.
3. Look for stuck or long-running jobs.

**Resolution:**
- Scale up workers via HPA or manual replicas.
- Identify and cancel stuck runs.

## WorkerDown

**Severity:** Critical

**Condition:** Worker missed two consecutive heartbeats (>10s).

**Investigation:**
1. Check pod logs: `kubectl logs <pod-name>`.
2. Check node health.
3. Review recent config changes.

**Resolution:**
- Restart pod if OOM or crash loop.
- Investigate persistent failure before scaling.

## CostSpike

**Severity:** Warning

**Condition:** Tenant cost rate exceeds 2x the 7-day rolling average.

**Investigation:**
1. Identify tenant via `ancora_tenant_cost_usd_total` labels.
2. Check for unusual run patterns (loop bugs, large prompts).
3. Verify quota settings for the tenant.

**Resolution:**
- Apply or tighten per-tenant cost quota.
- Notify tenant of unusual spend.

## ReplicationLag

**Severity:** Warning

**Condition:** Journal replication lag exceeds RPO target.

**Investigation:**
1. Check secondary replica connectivity.
2. Check network bandwidth between primary and secondary.
3. Review `DrConfig.rpo_secs` vs observed lag.

**Resolution:**
- Increase replication bandwidth.
- Trigger manual `replicate()` call.
- If lag persists, consider RPO/RTO trade-off adjustment.

## ResidencyViolation

**Severity:** Critical

**Condition:** Data written outside the tenant's allowed residency region.

**Investigation:**
1. Identify the tenant and the write operation from the audit log.
2. Determine if a config error routed to the wrong region.
3. Quarantine the affected data.

**Resolution:**
- Fix tenant residency config.
- Delete out-of-region copy after legal/compliance review.
- File incident report per compliance procedure.
