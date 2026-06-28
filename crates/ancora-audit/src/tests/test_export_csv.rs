use crate::{to_csv, AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn export_csv_has_header_row() {
    let log = ImmutableAuditLog::new();
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let csv = to_csv(&entries);
    assert!(csv.starts_with("id,tick,tenant_id,subject,operation,resource,outcome,severity\n"));
}
#[test]
fn export_csv_has_data_row() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 42, "t1", "bob", "write", "file.txt", Outcome::Failure, Severity::Warning));
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let csv = to_csv(&entries);
    assert!(csv.contains("bob"));
    assert!(csv.contains("write"));
    assert!(csv.contains("file.txt"));
}
