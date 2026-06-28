use crate::ImmutableAuditLog;
#[test]
fn empty_log_has_zero_count() {
    let log = ImmutableAuditLog::new();
    assert_eq!(log.count(), 0);
}
#[test]
fn empty_log_verify_all_is_true() {
    let log = ImmutableAuditLog::new();
    assert!(log.verify_all());
}
#[test]
fn empty_log_get_returns_none() {
    let log = ImmutableAuditLog::new();
    assert!(log.get(1).is_none());
}
