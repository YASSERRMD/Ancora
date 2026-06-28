use crate::Role;
#[test]
fn admin_dominates_all() {
    for r in Role::all() { assert!(Role::Admin.dominates(&r)); }
}
#[test]
fn viewer_only_dominates_itself() {
    assert!(Role::Viewer.dominates(&Role::Viewer));
    assert!(!Role::Viewer.dominates(&Role::Developer));
}
#[test]
fn operator_dominates_developer_and_viewer() {
    assert!(Role::Operator.dominates(&Role::Developer));
    assert!(Role::Operator.dominates(&Role::Viewer));
    assert!(!Role::Operator.dominates(&Role::Admin));
}
