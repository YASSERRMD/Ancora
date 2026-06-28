use crate::{default_permissions, Permission, Role};
#[test]
fn operator_can_read_secrets() {
    let p = default_permissions(&Role::Operator);
    assert!(p.contains(&Permission::SecretRead));
    assert!(p.contains(&Permission::SecretWrite));
}
#[test]
fn operator_cannot_assign_roles() {
    let p = default_permissions(&Role::Operator);
    assert!(!p.contains(&Permission::RoleAssign));
}
#[test]
fn operator_can_delete_agents() {
    let p = default_permissions(&Role::Operator);
    assert!(p.contains(&Permission::AgentDelete));
}
