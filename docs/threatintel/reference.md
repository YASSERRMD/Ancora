# ancora-threatintel Reference

Threat intelligence with IOC feeds, threat scoring, indicator management, and feed aggregation.

## Modules

### `indicator`
Core IOC type.

- `IndicatorKind`: `IpAddress | Domain | Url | FileHash | Email | CertificateHash`
- `ThreatLevel`: `Informational | Low | Medium | High | Critical` (orderable)
- `Indicator::new(id, tenant_id, kind, value, threat_level, source, observed_tick)`
- `.with_expiry(tick)`, `.with_tag(tag)`, `.with_metadata(k, v)`
- `.is_expired(tick)`, `.deactivate()`

### `feed`
Feed registration and indicator association.

- `FeedFormat`: `Stix | Taxii | Csv | Json | Internal`
- `ThreatFeed::new(id, tenant_id, name, format, source_url, tick)`
- `FeedStore`: `register_feed`, `get_feed`, `add_indicator_to_feed`, `indicators_for_feed`, `enabled_feeds`, `for_tenant`

### `score`
Threat scoring with recency decay.

- `ThreatScore::new(indicator_id, raw_score, confidence)` -- `is_actionable()` (score >= 40 && confidence >= 0.5)
- `ThreatScorer::score(indicator, recency_ticks, max_recency)` -> `ThreatScore`

### `store`
In-memory indicator store.

- `IndicatorStore`: `insert`, `get`, `get_mut`, `for_tenant`, `by_kind`, `by_threat_level`, `active`, `expired(tick)`, `by_value`, `count`

### `audit`
Immutable audit ledger.

- `ThreatIntelAction`: `IndicatorAdded | IndicatorExpired | IndicatorDeactivated | FeedIngested | FeedEnabled | FeedDisabled | ScoreComputed | AlertTriggered`
- `ThreatIntelAuditLog`: `record`, `count`, `for_tenant`, `by_action`, `all()`

### `alert`
Alert management.

- `AlertStatus`: `Open | Acknowledged | Suppressed | Closed`
- `ThreatAlert::new(id, tenant_id, indicator_id, threat_level, message, tick)`
- `AlertStore`: `add`, `get_mut`, `open`, `for_tenant`, `count`

### `stats`
Aggregate statistics.

- `ThreatIntelStats::for_tenant(indicators, tenant_id)` -> fields: `total_indicators, active_indicators, critical_count, high_count, by_kind, by_level`
- `is_critical_free()`

### `builder`
Fluent builder.

- `IndicatorBuilder::new(id, tenant_id, kind, value).threat_level().source().tick().expires_at().tag().build()`

### `query`
Filter indicators.

- `IndicatorQuery::new().kind().min_level().source().tag().active_only().run(iter)`

### `presets`
Ready-made IOCs and feeds.

- `known_bad_ip(tenant_id, tick)`, `known_malware_hash(tenant_id, tick)`, `phishing_domain(tenant_id, tick)`
- `internal_feed(tenant_id, tick)`

### `policy`
Threat policy evaluation.

- `PolicyDecision`: `Block | Alert | Monitor | Allow`
- `ThreatPolicy::new(tenant_id).block_threshold(f64).alert_threshold(f64).min_confidence(f64)`
- `.evaluate(score)`, `.should_block_indicator(indicator)`

### `summary`
Tenant health summary.

- `ThreatIntelSummary::generate(indicators, alerts, tenant_id)` -> `is_healthy` (no critical + no open alerts)

### `report`
Per-tenant cross-data report.

- `ThreatIntelReport::generate(indicators, feeds, alerts, audit, tenant_id, tick)`

## Quick start

```rust
use ancora_threatintel::builder::IndicatorBuilder;
use ancora_threatintel::indicator::{IndicatorKind, ThreatLevel};
use ancora_threatintel::score::ThreatScorer;
use ancora_threatintel::policy::ThreatPolicy;

let indicator = IndicatorBuilder::new("i1", "acme", IndicatorKind::IpAddress, "1.2.3.4")
    .threat_level(ThreatLevel::High)
    .source("threat-feed")
    .tick(1000)
    .build();

let score = ThreatScorer::score(&indicator, 0, 1000);
let policy = ThreatPolicy::new("acme");
let decision = policy.evaluate(&score);
println!("{:?}", decision);
```
