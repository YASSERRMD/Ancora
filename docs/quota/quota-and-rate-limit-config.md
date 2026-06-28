# Quota and Rate-Limit Configuration

## QuotaSchema

Each tenant gets a `QuotaSchema` that defines per-window limits:

| Field | Default | Description |
|-------|---------|-------------|
| `max_requests` | 1,000 | Maximum API requests per window |
| `max_tokens` | 1,000,000 | Maximum LLM tokens per window |
| `max_cost_usd` | 100.0 | Maximum cost (USD) per window |
| `window_secs` | 3,600 | Window length in seconds |
| `soft_limit_fraction` | 0.8 | Fraction of hard limit at which a warning is raised |

## Limit tiers

- **Soft limit**: raised as a `QuotaError::SoftLimitWarning`. Non-blocking. Caller should log and alert.
- **Hard limit**: raised as `QuotaError::HardLimitExceeded`. Blocking. Includes `retry_after_secs`.

## Per-provider rate limits

`ProviderRateCoordinator` applies a separate sliding-window counter per
(tenant, provider) pair. This prevents one tenant from exhausting provider
rate budgets that affect other tenants.

## Register a tenant

```rust
use ancora_quota::{QuotaEngine, QuotaSchema};

let mut engine = QuotaEngine::new();
engine.register_tenant("acme", QuotaSchema {
    max_requests: 500,
    max_tokens: 500_000,
    max_cost_usd: 50.0,
    window_secs: 3600,
    soft_limit_fraction: 0.8,
}, now_secs);
```

## Check before processing a request

```rust
match engine.check("acme", token_count, cost_usd, now_secs) {
    Ok(()) => { /* proceed */ }
    Err(QuotaError::SoftLimitWarning { metric, pct, .. }) => {
        log::warn!("{metric} at {pct:.0}% of limit");
        // proceed, but alert on-call
    }
    Err(QuotaError::HardLimitExceeded { retry_after_secs, .. }) => {
        return Err(/* HTTP 429 with Retry-After: {retry_after_secs} */);
    }
    _ => {}
}
```
