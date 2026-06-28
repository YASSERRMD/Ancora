# On-Call Guide

## Before your shift

- Confirm PagerDuty schedule shows you as primary
- Verify you have access to: kubectl, Grafana, Prometheus, ops CLI
- Review any open incidents or in-progress migrations from previous shift

## Responding to a page

1. Acknowledge within SLA (5 min for P1, 15 min for P2)
2. Assess severity: check `/healthz` and `/readyz`
3. Open incident with `Incident::new(id, title, severity, now, your_name)`
4. Look up the matching playbook from `ancora_runbook::catalog`
5. Run through playbook steps in order; do not skip
6. If the playbook does not resolve the issue, escalate via `default_policy_for(severity)`

## Useful commands during an incident

```bash
# Check worker status
kubectl get pods -l app=ancora-worker

# Tail worker logs
kubectl logs -f deployment/ancora-worker

# Force drain a worker
kubectl annotate pod <pod> ancora.io/drain=true

# Cancel a stuck run (via opscli)
ancora-opscli run cancel <run-id>

# Check circuit breaker state
# (no runtime CLI yet; check metrics: ancora_circuit_open gauge)
```

## Handing off to next on-call

1. Update incident summary with current status
2. Document the last 3 actions taken and results
3. List any pending action items
4. Verbally brief the incoming on-call if P1/P2 still active

## Post-mortem requirements

All P1 and P2 incidents require a post-mortem within 72 hours. Use the `PostMortem` struct from `ancora-runbook` and follow the blameless format:
- Impact summary
- Timeline (using `add_event`)
- Root cause (single sentence)
- Contributing factors (list)
- Action items with owners and due dates (using `add_action`)
