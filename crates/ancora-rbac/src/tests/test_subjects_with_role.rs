use crate::{AssignmentStore, Role, RoleAssignment};
#[test]
fn subjects_with_role_correct() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t1", Role::Developer, 0));
    store.assign(RoleAssignment::new("bob", "t1", Role::Developer, 0));
    store.assign(RoleAssignment::new("carol", "t1", Role::Admin, 0));
    let mut devs = store.subjects_with_role(&Role::Developer, "t1");
    devs.sort();
    assert_eq!(devs, vec!["alice", "bob"]);
}
#[test]
fn subjects_with_role_empty_when_none() {
    let store = AssignmentStore::new();
    assert!(store.subjects_with_role(&Role::Admin, "t1").is_empty());
}
