use crate::audit::{EnterpriseAction, EnterpriseAuditEntry, EnterpriseAuditLog};

fn entry(tick: u64, tenant_id: &str, action: EnterpriseAction) -> EnterpriseAuditEntry {
    EnterpriseAuditEntry::new(tick, tenant_id, action, "operator", "detail")
}

#[test]
fn empty_log() {
    let log = EnterpriseAuditLog::new();
    assert_eq!(log.count(), 0);
}

#[test]
fn record_and_count() {
    let mut log = EnterpriseAuditLog::new();
    log.record(entry(1, "t1", EnterpriseAction::LicenseIssued));
    log.record(entry(2, "t1", EnterpriseAction::IncidentOpened));
    assert_eq!(log.count(), 2);
}

#[test]
fn for_tenant() {
    let mut log = EnterpriseAuditLog::new();
    log.record(entry(1, "t1", EnterpriseAction::LicenseIssued));
    log.record(entry(2, "t2", EnterpriseAction::LicenseIssued));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("none").len(), 0);
}

#[test]
fn by_action() {
    let mut log = EnterpriseAuditLog::new();
    log.record(entry(1, "t1", EnterpriseAction::CheckpointRun));
    log.record(entry(2, "t1", EnterpriseAction::CheckpointRun));
    log.record(entry(3, "t1", EnterpriseAction::ReportGenerated));
    assert_eq!(log.by_action(&EnterpriseAction::CheckpointRun).len(), 2);
    assert_eq!(log.by_action(&EnterpriseAction::ReportGenerated).len(), 1);
}

#[test]
fn all_iterator() {
    let mut log = EnterpriseAuditLog::new();
    log.record(entry(1, "t1", EnterpriseAction::PostureAssessed));
    assert_eq!(log.all().count(), 1);
}
