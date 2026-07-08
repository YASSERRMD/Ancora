use crate::audit::{IncidentAction, IncidentAuditEntry, IncidentAuditLog};

#[test]
fn for_tenant_returns_matching() {
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
        IncidentAction::Escalated,
        "a",
        "",
    ));
    log.record(IncidentAuditEntry::new(
        3,
        "i3",
        "t2",
        IncidentAction::Created,
        "b",
        "",
    ));
    let t1_entries = log.for_tenant("t1");
    assert_eq!(t1_entries.len(), 2);
    assert!(t1_entries.iter().all(|e| e.tenant_id == "t1"));
}

#[test]
fn for_tenant_no_match() {
    let mut log = IncidentAuditLog::new();
    log.record(IncidentAuditEntry::new(
        1,
        "i1",
        "t1",
        IncidentAction::Created,
        "a",
        "",
    ));
    assert_eq!(log.for_tenant("t99").len(), 0);
}
