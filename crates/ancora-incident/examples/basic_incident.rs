use ancora_incident::audit::{IncidentAction, IncidentAuditEntry, IncidentAuditLog};
use ancora_incident::builder::IncidentBuilder;
use ancora_incident::incident::Severity;
use ancora_incident::postmortem::Postmortem;
use ancora_incident::presets::{critical_escalation_policy, security_runbook};
use ancora_incident::store::IncidentStore;
use ancora_incident::summary::IncidentSummary;
use ancora_incident::timeline::{IncidentTimeline, TimelineEvent, TimelineEventKind};

fn main() {
    let mut store = IncidentStore::new();
    let mut timeline = IncidentTimeline::new();
    let mut audit = IncidentAuditLog::new();

    let mut incident =
        IncidentBuilder::new("INC-001", "acme", "Database connection pool exhausted")
            .severity(Severity::Critical)
            .tick(1000)
            .build();

    store.insert(incident.clone());
    timeline.add(TimelineEvent::new(
        "INC-001",
        TimelineEventKind::Detected,
        "monitor",
        "Alert fired",
        1000,
    ));
    audit.record(IncidentAuditEntry::new(
        1000,
        "INC-001",
        "acme",
        IncidentAction::Created,
        "monitor",
        "Auto-detected",
    ));

    if let Some(i) = store.get_mut("INC-001") {
        i.assign("alice");
        i.triage();
        incident = i.clone();
    }
    timeline.add(TimelineEvent::new(
        "INC-001",
        TimelineEventKind::Assigned,
        "alice",
        "Assigned to alice",
        1010,
    ));
    audit.record(IncidentAuditEntry::new(
        1010,
        "INC-001",
        "acme",
        IncidentAction::Assigned,
        "alice",
        "alice picked up",
    ));

    let mut runbook = security_runbook("INC-001");
    if let Some(s) = runbook.get_step_mut("s1") {
        s.complete(1020);
    }
    if let Some(s) = runbook.get_step_mut("s2") {
        s.complete(1030);
    }

    let policy = critical_escalation_policy("acme");
    println!("Escalation levels: {}", policy.level_count());

    if let Some(i) = store.get_mut("INC-001") {
        i.resolve(1500);
        incident = i.clone();
    }
    timeline.add(TimelineEvent::new(
        "INC-001",
        TimelineEventKind::Resolved,
        "alice",
        "Issue resolved",
        1500,
    ));

    let pm = Postmortem::generate(
        &incident,
        Some(&runbook),
        &timeline,
        1500,
        "Connection pool config error",
        "Increased pool size and added monitoring",
    );
    println!(
        "Postmortem: {} duration_ticks={} completion_rate={:.0}%",
        pm.incident_id,
        pm.duration_ticks,
        pm.runbook_completion_rate() * 100.0
    );

    let all = vec![&incident];
    let summary = IncidentSummary::generate(&all, "acme");
    println!(
        "Summary: total={} active={} healthy={}",
        summary.total,
        summary.active_count,
        summary.is_healthy()
    );
}
