# Alert Tuning Guide

## Reducing false positives

### HighErrorRate

Default threshold: 5% over 5 minutes. If your workload is bursty, raise to 10% or extend the window to 15 minutes. Use `check_high_error_rate(error_rate, threshold, now)` with your tuned threshold.

### QueueBacklog

Default threshold: 100. Scale proportionally to your baseline queue depth. A rule of thumb: alert at 2x the 95th percentile queue depth observed in normal operation.

### CostSpike

Default: 2x the 7-day rolling average. For predictable workloads, lower to 1.5x. For spiky workloads, raise to 3x.

## Preventing alert fatigue

1. **Set appropriate cooldowns.** Use `AlertDedup::new(cooldown_secs)` with a cooldown matching your MTTD target. 5 minutes (300s) is a sensible default.
2. **Use maintenance windows** for planned maintenance and releases. Register them with `SilenceRegistry`.
3. **Tier your alerts.** Only `Critical` alerts should page on-call at night; `Warning` can wait for business hours.
4. **Every alert must have a runbook.** Verify with `rule.has_runbook()`.

## Severity guidelines

| Rule | Default | Raise when | Lower when |
|------|---------|-----------|-----------|
| HighErrorRate | Critical | Single provider temporary blip | Directly impacts SLO |
| QueueBacklog | Warning | Autoscaler handles it | Queue never drains |
| WorkerDown | Critical | Single worker in large pool | All workers affected |
| CostSpike | Warning | Tenant-only, no SLO impact | Budget alert |
| ReplicationLag | Warning | Within recovery window | Approaching RPO |
| ResidencyViolation | Critical | Always | Never lower |
