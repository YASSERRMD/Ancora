use crate::audit::{IncidentAction, IncidentAuditEntry, IncidentAuditLog};

#[test]
fn audit_record_and_count() {
    let mut log = IncidentAuditLog::new();
    log.record(IncidentAuditEntry::new(
        1,
        "i1",
        "t1",
        IncidentAction::Created,
        "alice",
        "",
    ));
    log.record(IncidentAuditEntry::new(
        2,
        "i1",
        "t1",
        IncidentAction::Assigned,
        "alice",
        "",
    ));
    assert_eq!(log.count(), 2);
}

#[test]
fn audit_for_incident() {
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
        "i2",
        "t1",
        IncidentAction::Created,
        "a",
        "",
    ));
    assert_eq!(log.for_incident("i1").len(), 1);
    assert_eq!(log.for_incident("i2").len(), 1);
}

#[test]
fn audit_for_tenant() {
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
        "t2",
        IncidentAction::Resolved,
        "b",
        "",
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t2").len(), 1);
}

#[test]
fn audit_all() {
    let mut log = IncidentAuditLog::new();
    for k in 0..5u64 {
        log.record(IncidentAuditEntry::new(
            k,
            "i1",
            "t1",
            IncidentAction::StatusUpdated,
            "a",
            "",
        ));
    }
    assert_eq!(log.all().count(), 5);
}
