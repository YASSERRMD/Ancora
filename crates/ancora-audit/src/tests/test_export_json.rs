use crate::{to_json, AuditEntry, ImmutableAuditLog, Outcome, Severity};
#[test]
fn export_json_contains_id_and_subject() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "alice", "read", "r", Outcome::Success, Severity::Info));
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let json = to_json(&entries);
    assert!(json.contains("\"subject\":\"alice\""));
    assert!(json.contains("\"operation\":\"read\""));
}
#[test]
fn export_json_is_array_format() {
    let mut log = ImmutableAuditLog::new();
    log.append(AuditEntry::new(0, 1, "t1", "a", "op", "r", Outcome::Success, Severity::Info));
    let entries: Vec<&AuditEntry> = log.entries().collect();
    let json = to_json(&entries);
    assert!(json.starts_with('[') && json.ends_with(']'));
}
