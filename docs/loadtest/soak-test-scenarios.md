# Soak Test Scenarios

## Baseline

- `target_rps`: 100
- `duration_secs`: 1800 (30 min)
- `concurrency`: 10
- SLO: error rate < 0.1%, p99 < 500ms

Validates stable throughput and memory under sustained load. Watch for memory growth and queue depth creep.

## Spike

- `target_rps`: 500 (5x baseline)
- `duration_secs`: 60
- `concurrency`: 50
- SLO: error rate < 5%, p99 < 2000ms

Validates the system does not cascade-fail under sudden burst. Circuit breakers and backpressure should activate gracefully.

## Recovery

1. Run baseline for 10 minutes.
2. Inject 60s of 100% error rate.
3. Restore baseline.
4. Confirm p99 returns to SLO within 2 minutes.

This validates that the `CircuitBreaker` closes and `ProviderFailover` promotes the secondary correctly.

## Memory soak

- `target_rps`: 50
- `duration_secs`: 7200 (2 hours)
- `payload_size_bytes`: 4096

Monitors for RSS growth over time. Acceptable drift: < 10% per hour.

## Draining under load

1. Start baseline workload.
2. At T+5min, drain one of 3 workers.
3. Confirm no request failures during drain.
4. Confirm drained worker shows no new allocations.
