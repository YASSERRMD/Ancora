use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, PolicyDecision};
#[test]
fn all_iterator_covers_every_record() {
    let mut log = NetpolAuditLog::new();
    let req = ConnectionRequest::tcp("t1", "a", "h", 443);
    log.record(EvaluationRecord::from(1, &req, &PolicyDecision::Allow));
    log.record(EvaluationRecord::from(
        2,
        &req,
        &PolicyDecision::Deny("d".into()),
    ));
    assert_eq!(log.all().count(), 2);
}
