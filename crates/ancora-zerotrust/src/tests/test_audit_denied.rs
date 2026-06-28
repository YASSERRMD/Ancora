use crate::audit::{ZtAction, ZtAuditEntry, ZtAuditLog};

#[test]
fn denied_entries_only_failed() {
    let mut log = ZtAuditLog::new();
    log.record(ZtAuditEntry::new(1, "t1", "i1", "r", ZtAction::AccessGranted, true, "ok"));
    log.record(ZtAuditEntry::new(2, "t1", "i1", "r", ZtAction::AccessDenied, false, "blocked"));
    log.record(ZtAuditEntry::new(3, "t1", "i1", "r", ZtAction::MfaRequired, false, "mfa"));
    let denied = log.denied();
    assert_eq!(denied.len(), 2);
    assert!(denied.iter().all(|e| !e.success));
}
