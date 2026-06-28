# Ancora Production Runbook

This document provides the top-level reference for on-call engineers responding to Ancora incidents.

## Incident severity classification

| Severity | Description | Response time |
|----------|-------------|---------------|
| P1 | Total service outage | 5 min |
| P2 | Major feature degradation | 15 min |
| P3 | Minor degradation, some users affected | 1 hour |
| P4 | Low impact, workaround available | Next business day |

## Escalation policy

Use `default_policy_for(severity)` from `ancora-runbook` to get the appropriate escalation tiers.

| Severity | Tier 1 | Tier 2 | Tier 3 |
|----------|--------|--------|--------|
| P1 | On-call primary (0s) | On-call secondary (5min) | EM (15min) |
| P2 | On-call primary (0s) | On-call secondary (15min) | - |
| P3/P4 | On-call primary (0s) | - | - |

## Standard playbooks

All playbooks are in `ancora_runbook::catalog`:
- `high_error_rate()` - error rate > 5%
- `worker_down()` - missed heartbeats
- `queue_backlog()` - queue depth spike

## First 5 minutes checklist (P1/P2)

1. Acknowledge the alert (PagerDuty or similar)
2. Check liveness probe: `GET /healthz`
3. Check readiness probe: `GET /readyz`
4. Check Prometheus: error rate, queue depth, worker utilization
5. Determine affected tenant(s)
6. Page on-call secondary if not resolved in 5 minutes (P1) or 15 minutes (P2)

## Post-incident

Use `PostMortem` from `ancora-runbook` to structure the blameless post-mortem. Assign `ActionItem`s with owners and due dates. Follow up weekly until all actions are completed.
