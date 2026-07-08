use crate::{default_permissions, Permission, Role};
#[test]
fn two_calls_return_same_set() {
    assert_eq!(
        default_permissions(&Role::Admin),
        default_permissions(&Role::Admin)
    );
}
#[test]
fn viewer_set_does_not_contain_admin_perms() {
    let v = default_permissions(&Role::Viewer);
    assert!(!v.contains(&Permission::TenantAdmin));
}
