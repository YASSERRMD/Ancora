use crate::{AssignmentStore, AuthzDecision, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};
#[test]
fn admin_in_t1_denied_in_t2() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("eve", "t1", Role::Admin, 0));
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    assert!(matches!(checker.check("eve", "t2", &Permission::AgentRead), AuthzDecision::Deny(_)));
}
#[test]
fn viewer_in_t2_cannot_act_in_t1() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("frank", "t2", Role::Viewer, 0));
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    assert!(!checker.is_allowed("frank", "t1", &Permission::AgentRead));
}
