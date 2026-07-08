use crate::{AssignmentStore, Permission, RbacGuard, Role, RoleAssignment, RolePolicy};
#[test]
fn guard_assert_permission_ok() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t1", Role::Developer, 0));
    let policy = RolePolicy::new();
    let guard = RbacGuard::new(&store, &policy);
    assert!(guard
        .assert_permission("alice", "t1", &Permission::AgentWrite)
        .is_ok());
}
#[test]
fn guard_assert_permission_err() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("viewer", "t1", Role::Viewer, 0));
    let policy = RolePolicy::new();
    let guard = RbacGuard::new(&store, &policy);
    assert!(guard
        .assert_permission("viewer", "t1", &Permission::AgentDelete)
        .is_err());
}
#[test]
fn guard_minimum_role_ok() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("op", "t1", Role::Operator, 0));
    let policy = RolePolicy::new();
    let guard = RbacGuard::new(&store, &policy);
    assert!(guard
        .assert_minimum_role("op", "t1", &Role::Developer)
        .is_ok());
}
