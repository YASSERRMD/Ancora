use crate::{AssignmentStore, Role, RoleAssignment};
#[test]
fn revoke_removes_assignment() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("carol", "t1", Role::Operator, 0));
    assert!(store.revoke("carol", "t1"));
    assert!(store.role_of("carol", "t1").is_none());
}
#[test]
fn revoke_nonexistent_returns_false() {
    let mut store = AssignmentStore::new();
    assert!(!store.revoke("nobody", "t1"));
}
