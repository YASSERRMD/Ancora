use crate::{AssignmentStore, AuthzDecision, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};
fn setup() -> (AssignmentStore, RolePolicy) {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t1", Role::Developer, 0));
    store.assign(RoleAssignment::new("bob", "t1", Role::Admin, 0));
    (store, RolePolicy::new())
}
#[test]
fn developer_allowed_agent_write() {
    let (store, policy) = setup();
    let checker = PermissionChecker::new(&store, &policy);
    assert_eq!(checker.check("alice", "t1", &Permission::AgentWrite), AuthzDecision::Allow);
}
#[test]
fn developer_denied_role_assign() {
    let (store, policy) = setup();
    let checker = PermissionChecker::new(&store, &policy);
    assert!(matches!(checker.check("alice", "t1", &Permission::RoleAssign), AuthzDecision::Deny(_)));
}
#[test]
fn admin_allowed_everything() {
    let (store, policy) = setup();
    let checker = PermissionChecker::new(&store, &policy);
    assert_eq!(checker.check("bob", "t1", &Permission::TenantAdmin), AuthzDecision::Allow);
}
