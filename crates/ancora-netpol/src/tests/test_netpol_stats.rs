use crate::{ConnectionRequest, EvaluationRecord, NetpolAuditLog, NetpolStats, PolicyDecision};
#[test]
fn stats_from_log_counts_correctly() {
    let mut log = NetpolAuditLog::new();
    let req = ConnectionRequest::tcp("t1", "a", "h", 80);
    for _ in 0..4 {
        log.record(EvaluationRecord::from(1, &req, &PolicyDecision::Allow));
    }
    for _ in 0..6 {
        log.record(EvaluationRecord::from(
            2,
            &req,
            &PolicyDecision::Deny("d".into()),
        ));
    }
    let stats = NetpolStats::from_log(&log, "t1");
    assert_eq!(stats.total, 10);
    assert_eq!(stats.allowed, 4);
    assert_eq!(stats.denied, 6);
    assert!((stats.deny_rate() - 0.6).abs() < 1e-10);
}
#[test]
fn global_stats_covers_all_tenants() {
    let mut log = NetpolAuditLog::new();
    let req1 = ConnectionRequest::tcp("t1", "a", "h", 80);
    let req2 = ConnectionRequest::tcp("t2", "b", "h", 80);
    log.record(EvaluationRecord::from(1, &req1, &PolicyDecision::Allow));
    log.record(EvaluationRecord::from(
        2,
        &req2,
        &PolicyDecision::Deny("d".into()),
    ));
    let stats = NetpolStats::global(&log);
    assert_eq!(stats.total, 2);
}
#[test]
fn stats_empty_log_returns_zeros() {
    let log = NetpolAuditLog::new();
    let stats = NetpolStats::from_log(&log, "t1");
    assert_eq!(stats.total, 0);
    assert!((stats.deny_rate()).abs() < 1e-10);
}
