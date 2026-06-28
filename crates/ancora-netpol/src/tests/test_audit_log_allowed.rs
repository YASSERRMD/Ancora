use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, PolicyDecision};
#[test]
fn allowed_for_returns_only_allowed_records() {
    let mut log = NetpolAuditLog::new();
    let req = ConnectionRequest::tcp("t1", "a", "safe.com", 443);
    log.record(EvaluationRecord::from(1, &req, &PolicyDecision::Allow));
    log.record(EvaluationRecord::from(2, &req, &PolicyDecision::Deny("denied".into())));
    assert_eq!(log.allowed_for("t1").len(), 1);
}
