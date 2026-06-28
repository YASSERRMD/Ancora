use crate::incident::{Incident, IncidentStatus, Severity};

#[test]
fn new_incident_is_open() {
    let i = Incident::new("INC-001", "High error rate", Severity::P1, 1000, "alice");
    assert_eq!(i.status, IncidentStatus::Open);
    assert!(!i.is_resolved());
}

#[test]
fn resolve_sets_resolved_at() {
    let mut i = Incident::new("INC-001", "test", Severity::P2, 1000, "bob");
    i.resolve(1600);
    assert!(i.is_resolved());
    assert_eq!(i.ttm_secs(), Some(600));
}

#[test]
fn mitigate_sets_mitigated_status() {
    let mut i = Incident::new("INC-002", "test", Severity::P3, 0, "carol");
    i.mitigate();
    assert_eq!(i.status, IncidentStatus::Mitigated);
}

#[test]
fn unresolved_ttm_is_none() {
    let i = Incident::new("INC-003", "test", Severity::P4, 0, "dan");
    assert_eq!(i.ttm_secs(), None);
}
