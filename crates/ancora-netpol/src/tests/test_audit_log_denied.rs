use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, PolicyDecision};
#[test]
fn denied_for_returns_only_denied_records() {
    let mut log = NetpolAuditLog::new();
    let req1 = ConnectionRequest::tcp("t1", "a", "api.com", 443);
    let req2 = ConnectionRequest::tcp("t1", "b", "bad.com", 80);
    log.record(EvaluationRecord::from(1, &req1, &PolicyDecision::Allow));
    log.record(EvaluationRecord::from(2, &req2, &PolicyDecision::Deny("blocked".into())));
    assert_eq!(log.denied_for("t1").len(), 1);
    assert_eq!(log.denied_for("t1")[0].host, "bad.com");
}
