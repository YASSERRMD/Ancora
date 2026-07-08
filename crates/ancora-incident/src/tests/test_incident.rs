use crate::incident::{Incident, IncidentStatus, Severity};

#[test]
fn new_incident_detected() {
    let i = Incident::new("i1", "t1", "Database outage", Severity::High, 100);
    assert_eq!(i.id, "i1");
    assert_eq!(i.severity, Severity::High);
    assert_eq!(i.status, IncidentStatus::Detected);
    assert!(i.is_active());
}

#[test]
fn resolve_incident() {
    let mut i = Incident::new("i1", "t1", "Test", Severity::Low, 100);
    i.resolve(200);
    assert!(!i.is_active());
    assert_eq!(i.resolved_tick, Some(200));
    assert_eq!(i.status, IncidentStatus::Resolved);
}

#[test]
fn duration_resolved() {
    let mut i = Incident::new("i1", "t1", "Test", Severity::Medium, 50);
    i.resolve(150);
    assert_eq!(i.duration(999), 100);
}

#[test]
fn duration_active() {
    let i = Incident::new("i1", "t1", "Test", Severity::Critical, 50);
    assert_eq!(i.duration(150), 100);
}

#[test]
fn status_transitions() {
    let mut i = Incident::new("i1", "t1", "Test", Severity::High, 1);
    i.triage();
    assert_eq!(i.status, IncidentStatus::Triaged);
    i.investigate();
    assert_eq!(i.status, IncidentStatus::Investigating);
    i.mitigate();
    assert_eq!(i.status, IncidentStatus::Mitigating);
    i.close();
    assert_eq!(i.status, IncidentStatus::Closed);
    assert!(!i.is_active());
}

#[test]
fn assign_incident() {
    let mut i = Incident::new("i1", "t1", "Test", Severity::Low, 1);
    assert!(i.assignee.is_none());
    i.assign("alice");
    assert_eq!(i.assignee.as_deref(), Some("alice"));
}

#[test]
fn with_metadata() {
    let i = Incident::new("i1", "t1", "Test", Severity::Low, 1).with_metadata("env", "prod");
    assert_eq!(i.metadata.get("env").map(|s| s.as_str()), Some("prod"));
}
