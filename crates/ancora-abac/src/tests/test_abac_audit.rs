use crate::{AbacAuditLog, AbacDecisionRecord, Decision};
#[test]
fn audit_records_allow() {
    let mut log = AbacAuditLog::new(100);
    log.record(AbacDecisionRecord::from_decision("read", &Decision::Allow, 1));
    assert_eq!(log.count(), 1);
    assert_eq!(log.deny_count(), 0);
}
#[test]
fn audit_records_deny() {
    let mut log = AbacAuditLog::new(100);
    log.record(AbacDecisionRecord::from_decision("write", &Decision::Deny("blocked".into()), 2));
    assert_eq!(log.deny_count(), 1);
}
#[test]
fn audit_capped() {
    let mut log = AbacAuditLog::new(2);
    for i in 0..5u64 { log.record(AbacDecisionRecord::from_decision("read", &Decision::Allow, i)); }
    assert_eq!(log.count(), 2);
}
