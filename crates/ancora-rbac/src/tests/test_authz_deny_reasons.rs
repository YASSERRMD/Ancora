use crate::{AssignmentStore, AuthzDecision, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};
#[test]
fn deny_message_contains_subject() {
    let store = AssignmentStore::new();
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    match checker.check("unknown-user", "t1", &Permission::AgentRead) {
        AuthzDecision::Deny(msg) => assert!(msg.contains("unknown-user")),
        _ => panic!("expected deny"),
    }
}
#[test]
fn deny_message_for_insufficient_role() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("viewer-user", "t1", Role::Viewer, 0));
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    match checker.check("viewer-user", "t1", &Permission::RoleAssign) {
        AuthzDecision::Deny(msg) => assert!(msg.contains("viewer")),
        _ => panic!("expected deny"),
    }
}
