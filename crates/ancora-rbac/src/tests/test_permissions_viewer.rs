use crate::{default_permissions, Permission, Role};
#[test]
fn viewer_can_read_agents() {
    let p = default_permissions(&Role::Viewer);
    assert!(p.contains(&Permission::AgentRead));
}
#[test]
fn viewer_cannot_write_agents() {
    let p = default_permissions(&Role::Viewer);
    assert!(!p.contains(&Permission::AgentWrite));
}
#[test]
fn viewer_can_read_audit() {
    let p = default_permissions(&Role::Viewer);
    assert!(p.contains(&Permission::AuditRead));
}
