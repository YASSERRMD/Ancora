use crate::{AssignmentStore, Role, RoleAssignment};
#[test]
fn assign_and_retrieve() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t1", Role::Developer, 0));
    assert_eq!(store.role_of("alice", "t1"), Some(&Role::Developer));
}
#[test]
fn unknown_subject_returns_none() {
    let store = AssignmentStore::new();
    assert!(store.role_of("nobody", "t1").is_none());
}
#[test]
fn cross_tenant_isolation() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t1", Role::Admin, 0));
    store.assign(RoleAssignment::new("alice", "t2", Role::Viewer, 0));
    assert_eq!(store.role_of("alice", "t1"), Some(&Role::Admin));
    assert_eq!(store.role_of("alice", "t2"), Some(&Role::Viewer));
}
