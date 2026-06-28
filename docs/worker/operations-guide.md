# Worker and Scheduler Operations Guide

The `ancora-worker` crate provides stateless workers that claim and execute runs
from the control plane journal using time-bounded leases.

## Architecture

Workers are entirely stateless. All durable state lives in the `ControlPlaneStore`
(or a persistent backend in production). A worker claims a run, executes one step,
appends the result to the journal, and releases the lease. If the worker crashes
before releasing, the lease expires and the run is automatically requeued.

## Worker lifecycle

1. **Register**: worker calls `register_worker(concurrency)` to get a worker ID.
2. **Claim**: `claim_run(worker_id)` atomically takes the highest-priority queued run.
3. **Execute**: the step function is invoked with the run context.
4. **Journal**: the step result is appended to the run's journal.
5. **Release**: `release_lease(worker_id, run_id, success)` marks the run completed or requeues it.
6. **Heartbeat**: `heartbeat_worker(worker_id)` extends the lease during long steps.

## Lease model

Leases expire after 30 seconds by default. Call `expire_leases()` periodically
(or in a background thread) to detect crashed workers and requeue their runs.
A run that fails more than five times is quarantined and not retried.

## Priority lanes

Runs are served in order: Critical > High > Normal > Low. Within a lane,
the `FairScheduler` balances across tenants by weight.

## Backpressure

When queue depth >= 2x total capacity (workers x concurrency), the scheduler
emits `Backpressure::Hard`. The control plane should stop accepting new runs or
scale up workers.

## Graceful shutdown

Call `pool.start_drain()` to stop claiming new runs. The pool will complete all
in-flight steps, then become idle. Use `ShutdownSignal` to coordinate across
threads.

## Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `concurrency_per_worker` | 4 | Max simultaneous runs per worker |
| `lease_duration_secs` | 30 | Seconds before a lease expires |
| `poison_threshold` | 5 | Failures before quarantine |
