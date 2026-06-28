use crate::{AuditEntryBuilder, Outcome, Severity};
#[test]
fn builder_produces_valid_entry() {
    let entry = AuditEntryBuilder::new(100, "t1", "alice")
        .operation("login")
        .resource("auth-service")
        .outcome(Outcome::Success)
        .severity(Severity::Info)
        .build();
    assert_eq!(entry.operation, "login");
    assert_eq!(entry.resource, "auth-service");
    assert_eq!(entry.outcome, Outcome::Success);
    assert!(entry.verify());
}
#[test]
fn builder_with_details_verifies() {
    let entry = AuditEntryBuilder::new(200, "t2", "bob")
        .operation("delete")
        .resource("file.txt")
        .outcome(Outcome::Blocked)
        .severity(Severity::Warning)
        .detail("reason", "insufficient-permission")
        .build();
    assert_eq!(entry.details.get("reason").unwrap(), "insufficient-permission");
    assert!(entry.verify());
}
