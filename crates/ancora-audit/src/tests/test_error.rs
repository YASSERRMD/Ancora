use crate::AuditError;
#[test]
fn error_checksum_mismatch_display() {
    let e = AuditError::ChecksumMismatch { id: 42 };
    let s = format!("{}", e);
    assert!(s.contains("42"));
}
#[test]
fn error_not_found_display() {
    let e = AuditError::EntryNotFound { id: 99 };
    let s = format!("{}", e);
    assert!(s.contains("99"));
}
#[test]
fn error_at_capacity_display() {
    let e = AuditError::LogAtCapacity { max: 1000 };
    let s = format!("{}", e);
    assert!(s.contains("1000"));
}
