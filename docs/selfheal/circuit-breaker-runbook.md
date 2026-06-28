# Circuit Breaker Runbook

## States

| State | Description |
|-------|-------------|
| Closed | Normal operation. Requests flow through. |
| Open | Too many failures. Requests are rejected immediately. |
| HalfOpen | Reset timeout expired. One probe request allowed through. |

## Tuning

- `failure_threshold`: number of consecutive failures before opening. Start at 5.
- `reset_timeout_secs`: how long to stay open. Start at 60s; increase if the provider needs more recovery time.

## Diagnosing a stuck-open circuit

1. Check `cb.failure_count()` -- if it equals `failure_threshold`, it opened on the last failure.
2. Check `cb.is_open(now)` -- if still true, the reset timeout has not elapsed.
3. Call `cb.try_half_open(now)` manually to force a probe if you have confirmation the provider has recovered.
4. If the probe succeeds, call `cb.on_success()` to close.

## Provider failover interaction

When a circuit breaker opens, trigger `ProviderFailover::failover()` to switch to the next available provider. Only close the circuit and restore the original provider once health checks confirm recovery.
