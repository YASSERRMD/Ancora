use crate::Role;
#[test]
fn role_as_str_values() {
    assert_eq!(Role::Admin.as_str(), "admin");
    assert_eq!(Role::Operator.as_str(), "operator");
    assert_eq!(Role::Developer.as_str(), "developer");
    assert_eq!(Role::Viewer.as_str(), "viewer");
}
