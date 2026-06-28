use crate::{default_permissions, Permission, Role};
#[test]
fn developer_can_write_agents() {
    let p = default_permissions(&Role::Developer);
    assert!(p.contains(&Permission::AgentWrite));
    assert!(p.contains(&Permission::AgentExecute));
}
#[test]
fn developer_cannot_delete_secrets() {
    let p = default_permissions(&Role::Developer);
    assert!(!p.contains(&Permission::SecretDelete));
}
#[test]
fn developer_inherits_viewer_perms() {
    let p = default_permissions(&Role::Developer);
    assert!(p.contains(&Permission::AgentRead));
    assert!(p.contains(&Permission::TaskRead));
}
