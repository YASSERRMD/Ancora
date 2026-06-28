use crate::{default_permissions, Permission, Role};
#[test]
fn admin_can_assign_roles() {
    let p = default_permissions(&Role::Admin);
    assert!(p.contains(&Permission::RoleAssign));
    assert!(p.contains(&Permission::TenantAdmin));
}
#[test]
fn admin_can_delete_everything() {
    let p = default_permissions(&Role::Admin);
    assert!(p.contains(&Permission::AgentDelete));
    assert!(p.contains(&Permission::SecretDelete));
    assert!(p.contains(&Permission::UserDelete));
}
#[test]
fn admin_has_all_lower_perms() {
    let admin = default_permissions(&Role::Admin);
    for lower in [Role::Operator, Role::Developer, Role::Viewer] {
        for perm in default_permissions(&lower) {
            assert!(admin.contains(&perm), "admin missing {:?}", perm);
        }
    }
}
