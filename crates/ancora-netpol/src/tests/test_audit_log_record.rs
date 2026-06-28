use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, PolicyDecision};
#[test]
fn audit_log_records_entry() {
    let mut log = NetpolAuditLog::new();
    let req = ConnectionRequest::tcp("t1", "a", "api.com", 443);
    log.record(EvaluationRecord::from(1, &req, &PolicyDecision::Allow));
    assert_eq!(log.count(), 1);
}
