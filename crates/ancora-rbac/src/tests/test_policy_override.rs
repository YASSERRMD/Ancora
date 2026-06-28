use crate::{Permission, Role, RolePolicy};
#[test]
fn grant_extra_permission_to_viewer() {
    let mut policy = RolePolicy::new();
    policy.grant(Role::Viewer, Permission::SecretRead);
    let perms = policy.permissions_for(&Role::Viewer);
    assert!(perms.contains(&Permission::SecretRead));
}
#[test]
fn revoke_does_not_affect_base_perms() {
    let mut policy = RolePolicy::new();
    policy.revoke(&Role::Admin, &Permission::TenantAdmin);
    let perms = policy.permissions_for(&Role::Admin);
    assert!(perms.contains(&Permission::TenantAdmin));
}
