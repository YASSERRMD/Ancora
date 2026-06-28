use crate::{default_permissions, Role};
#[test]
fn admin_permission_count_largest() {
    let admin = default_permissions(&Role::Admin).len();
    for r in [Role::Operator, Role::Developer, Role::Viewer] {
        assert!(admin >= default_permissions(&r).len());
    }
}
