use crate::Permission;
#[test]
fn all_permission_strings_are_unique() {
    let perms = [
        Permission::AgentRead, Permission::AgentWrite, Permission::AgentDelete, Permission::AgentExecute,
        Permission::TaskRead, Permission::TaskWrite, Permission::TaskDelete,
        Permission::LogRead, Permission::LogWrite,
        Permission::SecretRead, Permission::SecretWrite, Permission::SecretDelete,
        Permission::PolicyRead, Permission::PolicyWrite,
        Permission::UserRead, Permission::UserWrite, Permission::UserDelete,
        Permission::RoleAssign, Permission::TenantAdmin, Permission::AuditRead,
    ];
    let mut strs: Vec<&str> = perms.iter().map(|p| p.as_str()).collect();
    strs.sort();
    strs.dedup();
    assert_eq!(strs.len(), perms.len());
}
#[test]
fn permission_strings_use_colon_separator() {
    let perms = [Permission::AgentRead, Permission::SecretWrite, Permission::TenantAdmin];
    for p in &perms { assert!(p.as_str().contains(':'), "expected colon in {}", p.as_str()); }
}
