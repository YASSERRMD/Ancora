use crate::{default_permissions, Permission, Role};
#[test]
fn viewer_cannot_write_any_resource() {
    let perms = default_permissions(&Role::Viewer);
    let write_perms = [
        Permission::AgentWrite,
        Permission::TaskWrite,
        Permission::LogWrite,
        Permission::SecretWrite,
        Permission::PolicyWrite,
        Permission::UserWrite,
    ];
    for p in &write_perms {
        assert!(!perms.contains(p), "viewer should not have {:?}", p);
    }
}
