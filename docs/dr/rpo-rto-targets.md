# RPO and RTO Targets and Assumptions

## Targets

| Target | Default | Description |
|--------|---------|-------------|
| RPO | 60s | Maximum acceptable data loss |
| RTO | 300s | Maximum time to restore service |
| Replication interval | 30s | How often secondary is synced |

## Assumptions

- Replication runs continuously, not just at failover time.
- The secondary is in the same availability zone as the primary by default.
  For regional DR, the secondary should be in a separate region.
- `replication_satisfies_rpo()` must return `true` at all times:
  `replication_interval_secs <= rpo_secs`.

## Adjusting targets

```rust
use ancora_dr::DRConfig;

let cfg = DRConfig {
    rpo_secs: 30,
    rto_secs: 120,
    replication_interval_secs: 15,
};
```

## Compliance

For regulated environments (e.g., financial, healthcare, government):
- RPO must be <= 15 minutes for most frameworks.
- RTO must be <= 4 hours for Tier 2, <= 1 hour for Tier 1.
- DR drills must be run quarterly and results recorded.
