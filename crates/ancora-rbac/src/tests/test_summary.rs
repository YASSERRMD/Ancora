use crate::summary::summarize;
use crate::{AssignmentStore, Role, RoleAssignment, RolePolicy};
#[test]
fn summary_has_correct_role() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("alice", "t", Role::Developer, 0));
    let policy = RolePolicy::new();
    let s = summarize(&store, &policy, "alice", "t");
    assert_eq!(s.role, Some("developer".to_string()));
    assert!(s.permission_count > 0);
}
#[test]
fn summary_no_role() {
    let store = AssignmentStore::new();
    let policy = RolePolicy::new();
    let s = summarize(&store, &policy, "nobody", "t");
    assert_eq!(s.role, None);
    assert_eq!(s.permission_count, 0);
}
