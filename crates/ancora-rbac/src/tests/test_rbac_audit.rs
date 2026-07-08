use crate::{Permission, RbacAuditLog};
#[test]
fn audit_records_denied() {
    let mut log = RbacAuditLog::new(100);
    log.record_check("alice", "t1", &Permission::TenantAdmin, false, 1);
    assert_eq!(log.denied_count(), 1);
}
#[test]
fn audit_records_granted() {
    let mut log = RbacAuditLog::new(100);
    log.record_check("alice", "t1", &Permission::AgentRead, true, 1);
    assert_eq!(log.count(), 1);
    assert_eq!(log.denied_count(), 0);
}
#[test]
fn audit_capped() {
    let mut log = RbacAuditLog::new(3);
    for _ in 0..5 {
        log.record_check("u", "t", &Permission::AgentRead, true, 0);
    }
    assert_eq!(log.count(), 3);
}
