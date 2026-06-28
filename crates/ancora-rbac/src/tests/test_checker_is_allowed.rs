use crate::{AssignmentStore, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};
#[test]
fn is_allowed_convenience_returns_bool() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("dev", "t", Role::Developer, 0));
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    assert!(checker.is_allowed("dev", "t", &Permission::AgentWrite));
    assert!(!checker.is_allowed("dev", "t", &Permission::TenantAdmin));
}
#[test]
fn require_role_dominates_developer_over_viewer() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("dev", "t", Role::Developer, 0));
    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&store, &policy);
    assert!(checker.require_role_dominates("dev", "t", &Role::Viewer) == crate::AuthzDecision::Allow);
    assert!(matches!(checker.require_role_dominates("dev", "t", &Role::Admin), crate::AuthzDecision::Deny(_)));
}
