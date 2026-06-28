# Reliability, Chaos, and Load Test Plan

This document describes the chaos, load, and reliability tests introduced in Phase 154. All tests run offline with no live network calls, no real process kills, and no actual disk writes.

## Chaos tests

| File | Coverage |
|---|---|
| `chaos_retry_backoff.rs` | Exponential back-off, cap at shift 10, immediate success |
| `chaos_network_fault.rs` | Timeout, connection refused, partial read, recovery |
| `chaos_process_kill.rs` | Kill at step N, resume from checkpoint, done steps not re-run |
| `chaos_oom_guard.rs` | Memory budget enforcement, free restores capacity |
| `chaos_clock_skew.rs` | Non-monotonic wall-clock timestamps tolerated during replay |
| `chaos_disk_full.rs` | Disk quota enforcement, failed write leaves used unchanged |
| `chaos_provider_failover.rs` | Primary down -- secondary serves; all down -- error |
| `chaos_journal_corruption.rs` | Checksum-based corruption detection per entry |
| `chaos_partial_write.rs` | Truncated last entry excluded from recovery count |

## Load tests

| File | Coverage |
|---|---|
| `load_throughput.rs` | 10k events processed within 2s |
| `load_concurrent_runs.rs` | 100 runs converge, unique run IDs, shorter vs longer runs |
| `load_token_stream.rs` | 50k tokens drained within 1s |
| `load_memory_store.rs` | 1k vector chunks insert + cosine search within 500ms |
| `load_replay_throughput.rs` | 5k recorded events replayed within 1s |

## Reliability tests

| File | Coverage |
|---|---|
| `reliability_circuit_breaker.rs` | Open/half-open/close transitions, probe, reopen on failure |
| `reliability_deadline.rs` | Per-run deadline abort, remaining_ns, exact boundary |
| `reliability_rate_limit.rs` | Token-bucket burst then rate-limited, refill, capacity cap |
| `reliability_graceful_shutdown.rs` | In-flight runs complete, new runs rejected |
| `reliability_health_check.rs` | Down takes priority over Degraded over Ok, empty list Ok |

## Running the suite

```bash
cargo test -p ancora-core \
  --test chaos_retry_backoff \
  --test chaos_network_fault \
  --test chaos_process_kill \
  --test chaos_oom_guard \
  --test chaos_clock_skew \
  --test chaos_disk_full \
  --test chaos_provider_failover \
  --test chaos_journal_corruption \
  --test chaos_partial_write \
  --test load_throughput \
  --test load_concurrent_runs \
  --test load_token_stream \
  --test load_memory_store \
  --test load_replay_throughput \
  --test reliability_circuit_breaker \
  --test reliability_deadline \
  --test reliability_rate_limit \
  --test reliability_graceful_shutdown \
  --test reliability_health_check
```

All tests are offline by default.
