use crate::{AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn verify_all_passes_on_clean_log() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "alice",
        "login",
        "auth",
        Outcome::Success,
        Severity::Info,
    ));
    log.append(AuditEntry::new(
        0,
        2,
        "t1",
        "bob",
        "logout",
        "auth",
        Outcome::Success,
        Severity::Info,
    ));
    assert!(log.verify_all());
}
#[test]
fn verify_all_fails_after_tamper() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(
        0,
        1,
        "t1",
        "alice",
        "login",
        "auth",
        Outcome::Success,
        Severity::Info,
    ));
    let entry = log.entries().next().unwrap();
    assert!(entry.verify());
    let mut tampered = entry.clone();
    tampered.operation = "delete".into();
    assert!(!tampered.verify());
}
