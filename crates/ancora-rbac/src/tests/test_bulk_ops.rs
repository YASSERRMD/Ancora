use crate::bulk::{assign_bulk, check_all, effective_role_set};
use crate::{AssignmentStore, Permission, Role, RoleAssignment, RolePolicy};
#[test]
fn bulk_assign_all_present() {
    let mut store = AssignmentStore::new();
    assign_bulk(
        &mut store,
        vec![
            RoleAssignment::new("a", "t", Role::Admin, 0),
            RoleAssignment::new("b", "t", Role::Viewer, 0),
        ],
    );
    assert_eq!(store.role_of("a", "t"), Some(&Role::Admin));
    assert_eq!(store.role_of("b", "t"), Some(&Role::Viewer));
}
#[test]
fn check_all_returns_correct_decisions() {
    let mut store = AssignmentStore::new();
    store.assign(RoleAssignment::new("dev", "t", Role::Developer, 0));
    let policy = RolePolicy::new();
    let results = check_all(
        &store,
        &policy,
        "dev",
        "t",
        &[Permission::AgentWrite, Permission::RoleAssign],
    );
    assert!(results[0] == crate::AuthzDecision::Allow);
    assert!(matches!(results[1], crate::AuthzDecision::Deny(_)));
}
#[test]
fn effective_role_set_for_admin() {
    let set = effective_role_set(&Role::Admin);
    assert_eq!(set.len(), 4);
}
