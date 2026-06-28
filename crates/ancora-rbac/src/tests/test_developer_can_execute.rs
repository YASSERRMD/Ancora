use crate::{default_permissions, Permission, Role};
#[test]
fn developer_can_execute_agents() {
    assert!(default_permissions(&Role::Developer).contains(&Permission::AgentExecute));
}
#[test]
fn viewer_cannot_execute() {
    assert!(!default_permissions(&Role::Viewer).contains(&Permission::AgentExecute));
}
