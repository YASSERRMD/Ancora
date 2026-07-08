// Security: RBAC -- role-based access control for run operations.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

struct RolePolicy {
    roles: BTreeMap<&'static str, Vec<Permission>>,
}

impl RolePolicy {
    fn new() -> Self {
        let mut roles: BTreeMap<&str, Vec<Permission>> = BTreeMap::new();
        roles.insert("viewer", vec![Permission::Read]);
        roles.insert("operator", vec![Permission::Read, Permission::Execute]);
        roles.insert("editor", vec![Permission::Read, Permission::Write]);
        roles.insert(
            "admin",
            vec![
                Permission::Read,
                Permission::Write,
                Permission::Execute,
                Permission::Admin,
            ],
        );
        Self { roles }
    }

    fn has_permission(&self, role: &str, perm: &Permission) -> bool {
        self.roles
            .get(role)
            .map(|perms| perms.contains(perm))
            .unwrap_or(false)
    }
}

#[test]
fn test_viewer_can_read() {
    let p = RolePolicy::new();
    assert!(p.has_permission("viewer", &Permission::Read));
}

#[test]
fn test_viewer_cannot_execute() {
    let p = RolePolicy::new();
    assert!(!p.has_permission("viewer", &Permission::Execute));
}

#[test]
fn test_operator_can_execute() {
    let p = RolePolicy::new();
    assert!(p.has_permission("operator", &Permission::Execute));
}

#[test]
fn test_operator_cannot_write() {
    let p = RolePolicy::new();
    assert!(!p.has_permission("operator", &Permission::Write));
}

#[test]
fn test_admin_has_all_permissions() {
    let p = RolePolicy::new();
    for perm in &[
        Permission::Read,
        Permission::Write,
        Permission::Execute,
        Permission::Admin,
    ] {
        assert!(p.has_permission("admin", perm));
    }
}

#[test]
fn test_unknown_role_has_no_permissions() {
    let p = RolePolicy::new();
    assert!(!p.has_permission("ghost", &Permission::Read));
}
