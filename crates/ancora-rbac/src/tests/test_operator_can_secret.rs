use crate::{default_permissions, Permission, Role};
#[test]
fn operator_can_manage_secrets() {
    let p = default_permissions(&Role::Operator);
    assert!(p.contains(&Permission::SecretRead));
    assert!(p.contains(&Permission::SecretWrite));
    assert!(!p.contains(&Permission::SecretDelete));
}
