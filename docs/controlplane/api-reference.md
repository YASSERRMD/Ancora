# Control Plane API Reference

The `ancora-controlplane` crate provides a management API for creating, inspecting, and operating runs and workers at scale. All operations require a bearer token and are testable offline without any network dependencies.

## Concepts

- **Run**: a unit of agent work with a priority, state, tenant scope, and cost record.
- **Worker**: a stateless executor that claims runs via a time-bounded lease.
- **Tenant**: the isolation boundary for runs, costs, and journal entries.

## Run states

```
Queued -> Assigned -> Running -> Completed
                   -> Paused  -> Queued (on resume)
                   -> Cancelled
Queued -> Cancelled
* -> Quarantined (on repeated failure)
```

## API surface

### Runs

| Operation | Description |
|-----------|-------------|
| `create(tenant, priority)` | Enqueue a new run |
| `get(id)` | Fetch a run by ID |
| `list(tenant?, state?, cursor, limit)` | Paginated run list |
| `cancel(id)` | Cancel a queued, assigned, running, or paused run |
| `pause(id)` | Pause a running run |
| `resume(id, decision)` | Resume or reject a paused run |
| `journal_tail(run_id, from_seq)` | Stream journal entries from a sequence number |
| `cost_per_run(run_id)` | Token and USD cost for one run |
| `cost_aggregate(tenant_id)` | Aggregate cost across all runs in a tenant |

### Workers

| Operation | Description |
|-----------|-------------|
| `register(concurrency)` | Register a new worker |
| `heartbeat(worker_id)` | Renew the worker's liveness timestamp and lease |
| `claim_run(worker_id)` | Atomically claim the highest-priority queued run |
| `release(worker_id, run_id, success)` | Complete or fail a run and release the lease |
| `expire_leases()` | Requeue runs whose worker leases have expired |

### Health

| Endpoint | Description |
|----------|-------------|
| `liveness()` | Always returns live=true if the process is up |
| `readiness()` | Returns ready=true when workers exist or the queue is empty |

## Authentication

All operations accept a `token: Option<&str>`. The `TokenAuth` struct stores SHA-256 hashes of valid tokens. Passing `None` or an empty string returns `AuthError::MissingToken`; an unrecognized token returns `AuthError::InvalidToken`. Tokens are never stored in plain text.

## Priority

Runs are queued in a max-heap ordered by `RunPriority` (Critical > High > Normal > Low), with earlier `created_at` breaking ties at equal priority.

## Lease model

When a worker claims a run, a lease is issued for 30 seconds. The worker must call `heartbeat` before expiry to extend the lease. If the lease expires, the control plane requeues the run and increments the failure count. After five failures the run is quarantined.

## Pagination

`list` returns a `Page<Run>` with an optional `next_cursor`. Pass the cursor back in the next call to retrieve the following page.
