use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, NetworkPolicy, PolicyDecision, PolicySummary};
#[test]
fn deny_rate_computed_from_audit_log() {
    let policy = NetworkPolicy::deny_by_default("t1");
    let mut log = NetpolAuditLog::new();
    let req = ConnectionRequest::tcp("t1", "a", "host", 80);
    for _ in 0..3 { log.record(EvaluationRecord::from(1, &req, &PolicyDecision::Deny("d".into()))); }
    for _ in 0..7 { log.record(EvaluationRecord::from(2, &req, &PolicyDecision::Allow)); }
    let summary = PolicySummary::from(&policy, &log);
    assert!((summary.deny_rate() - 0.3).abs() < 1e-10);
}
