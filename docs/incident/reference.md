# ancora-incident Reference

Incident response with automated runbooks, escalation policies, timeline tracking, and postmortem generation.

## Modules

### `incident`
Core `Incident` type.

- `Severity`: `Low | Medium | High | Critical` (orderable)
- `IncidentStatus`: `Detected | Triaged | Investigating | Mitigating | Resolved | Closed`
- `Incident::new(id, tenant_id, title, severity, detected_tick)`
- `incident.assign(name)`, `triage()`, `investigate()`, `mitigate()`, `resolve(tick)`, `close()`
- `incident.is_active()`, `duration(current_tick)`

### `runbook`
Runbook steps and execution tracking.

- `StepStatus`: `Pending | InProgress | Completed | Skipped | Failed`
- `RunbookStep::new(id, title, description)`, `complete(tick)`, `skip()`, `fail()`, `start()`
- `Runbook::new(id, name, incident_id)`, `add_step`, `is_complete()`, `progress()`, `get_step_mut(id)`

### `escalation`
Escalation policies and notification targets.

- `EscalationChannel`: `Pager | Email | Chat | Phone`
- `EscalationPolicy::new(tenant_id, min_severity)`, `add_level`, `should_escalate(&severity)`
- `EscalationLevel::new(level, on_call, channel, delay_ticks)`

### `timeline`
Timeline event tracking per incident.

- `TimelineEventKind`: `Detected | Assigned | StatusChanged | RunbookStepCompleted | EscalationTriggered | Note | Resolved`
- `TimelineEvent::new(incident_id, kind, author, detail, tick)`
- `IncidentTimeline`: `add`, `for_incident(id)`, `by_kind(kind)`, `all()`

### `store`
In-memory incident store.

- `IncidentStore`: `insert`, `get`, `get_mut`, `remove`, `for_tenant`, `active`, `by_severity`, `by_status`, `count`

### `postmortem`
Postmortem report generation.

- `Postmortem::generate(incident, runbook, timeline, tick, root_cause, remediation)`
- `runbook_completion_rate()` -> f64

### `audit`
Immutable audit log.

- `IncidentAction`: `Created | StatusUpdated | Assigned | Escalated | RunbookStarted | RunbookStepDone | Resolved | PostmortemCreated`
- `IncidentAuditLog`: `record`, `count`, `for_incident`, `for_tenant`, `all()`

### `stats`
Aggregate statistics per tenant.

- `IncidentStats::for_tenant(incidents, tenant_id, current_tick)` -> fields: `total, active, resolved, by_severity, mean_duration`

### `builder`
Fluent builders.

- `IncidentBuilder::new(id, tenant_id, title).severity().tick().build()`
- `RunbookBuilder::new(id, name, incident_id).step(id, title, desc).build()`

### `presets`
Ready-made runbooks and policies.

- `security_runbook(incident_id)` - 5-step security incident runbook
- `critical_escalation_policy(tenant_id)` - 3-level pager/phone escalation for High+

### `query`
Filter incidents.

- `IncidentQuery::new().severity(s).status(s).assignee(a).active_only().run(iter)`

### `report`
Per-incident report combining all data.

- `IncidentReport::generate(incident, runbook, timeline, audit, escalations, tick)`

### `summary`
Tenant-level health summary.

- `IncidentSummary::generate(incidents, tenant_id)` -> `is_healthy()` (no critical, no unassigned)

## Quick start

```rust
use ancora_incident::builder::IncidentBuilder;
use ancora_incident::incident::Severity;
use ancora_incident::store::IncidentStore;

let mut store = IncidentStore::new();
let i = IncidentBuilder::new("INC-001", "acme", "DB outage")
    .severity(Severity::Critical)
    .tick(1000)
    .build();
store.insert(i);
if let Some(i) = store.get_mut("INC-001") { i.resolve(1500); }
```
