use crate::audit::{IncidentAction, IncidentAuditEntry, IncidentAuditLog};

#[test]
fn for_incident_returns_matching() {
    let mut log = IncidentAuditLog::new();
    log.record(IncidentAuditEntry::new(
        1,
        "i1",
        "t1",
        IncidentAction::Created,
        "a",
        "",
    ));
    log.record(IncidentAuditEntry::new(
        2,
        "i1",
        "t1",
        IncidentAction::Assigned,
        "a",
        "",
    ));
    log.record(IncidentAuditEntry::new(
        3,
        "i2",
        "t1",
        IncidentAction::Created,
        "a",
        "",
    ));
    let i1_entries = log.for_incident("i1");
    assert_eq!(i1_entries.len(), 2);
    assert!(i1_entries.iter().all(|e| e.incident_id == "i1"));
}

#[test]
fn for_incident_empty_log() {
    let log = IncidentAuditLog::new();
    assert_eq!(log.for_incident("i1").len(), 0);
}
