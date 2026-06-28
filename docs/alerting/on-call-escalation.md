# On-Call Escalation Policy

## Severity matrix

| Severity | Response SLA | Escalation after |
|----------|-------------|-----------------|
| Critical | 15 minutes | 30 minutes |
| Warning | 2 hours | 8 hours |
| Info | Next business day | (none) |

## Escalation path

1. **Primary on-call** receives PagerDuty alert.
2. After 30 minutes without acknowledgement: **Secondary on-call** paged.
3. After 1 hour without resolution for Critical: **Engineering manager** paged.
4. After 2 hours for Critical customer impact: **Incident commander** declared.

## Alert routing

All alerts route to the webhook configured in `WebhookRouter`. The webhook
endpoint should forward to PagerDuty, Slack, or your on-call tool of choice.

```rust
let mut router = WebhookRouter::new("https://events.pagerduty.com/v2/enqueue");
// route fired alerts
if !silences.is_silenced(&alert.rule_name, now) && dedup.should_route(&alert) {
    router.route(alert);
}
```

## Maintenance windows

Register maintenance windows in `SilenceRegistry` before planned maintenance:

```rust
let mut silences = SilenceRegistry::default();
silences.add(MaintenanceWindow::new(
    "db-migration-2026-07-01",
    1751328000, // unix timestamp for window start
    1751334600, // window end (+110 min)
    None,       // silence all alerts
));
```

## Deduplication

Set a cooldown of 300 seconds (5 minutes) to prevent alert storms:

```rust
let mut dedup = AlertDedup::new(300);
```
