# Operator Runbook: Limit Incidents

## Alert: tenant hard limit exceeded

**Symptom**: `QuotaError::HardLimitExceeded` logged repeatedly for a tenant.

**Steps**:
1. Query current usage: `engine.usage("tenant-id", now)`.
2. Check whether the burst is legitimate (batch job, integration test, etc.).
3. If legitimate, temporarily raise `max_requests` or `max_cost_usd` in the tenant's `QuotaSchema`.
4. If not legitimate, suspend the tenant via `TenantRegistry::suspend`.

## Alert: soft limit warning volume rising

**Symptom**: `SoftLimitWarning` rate exceeds 1 per minute for a tenant.

**Steps**:
1. Review the tenant's run concurrency and token consumption trend.
2. If workload is growing normally, coordinate a quota upgrade with the tenant admin.
3. If abnormal (runaway agent), cancel in-flight runs via the control plane cancel API.

## Budget reset

Budgets reset automatically at the end of each `window_secs` period.
No manual action is needed unless a tenant needs an early reset (e.g. after
an erroneous bulk run).

To force an early reset, re-register the tenant with a fresh `QuotaSchema`
and a current `now` timestamp:
```rust
engine.register_tenant("acme", existing_schema, now_secs);
```

## Provider rate limits

If `ProviderLimitExceeded` appears, check whether:
- The provider's own rate limit has been reduced.
- Multiple tenants share the same provider key and are competing.
- The `max_rps` value in the coordinator needs to be raised.
