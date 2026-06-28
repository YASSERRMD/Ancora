use crate::permission::Permission;
use crate::role::Role;
use std::collections::{HashMap, HashSet};

pub fn default_permissions(role: &Role) -> HashSet<Permission> {
    let mut perms = HashSet::new();
    match role {
        Role::Viewer => {
            perms.insert(Permission::AgentRead);
            perms.insert(Permission::TaskRead);
            perms.insert(Permission::LogRead);
            perms.insert(Permission::PolicyRead);
            perms.insert(Permission::UserRead);
            perms.insert(Permission::AuditRead);
        }
        Role::Developer => {
            perms.extend(default_permissions(&Role::Viewer));
            perms.insert(Permission::AgentWrite);
            perms.insert(Permission::AgentExecute);
            perms.insert(Permission::TaskWrite);
            perms.insert(Permission::TaskDelete);
            perms.insert(Permission::LogWrite);
        }
        Role::Operator => {
            perms.extend(default_permissions(&Role::Developer));
            perms.insert(Permission::AgentDelete);
            perms.insert(Permission::SecretRead);
            perms.insert(Permission::SecretWrite);
            perms.insert(Permission::PolicyWrite);
            perms.insert(Permission::UserWrite);
        }
        Role::Admin => {
            perms.extend(default_permissions(&Role::Operator));
            perms.insert(Permission::SecretDelete);
            perms.insert(Permission::UserDelete);
            perms.insert(Permission::RoleAssign);
            perms.insert(Permission::TenantAdmin);
        }
    }
    perms
}

#[derive(Debug, Default)]
pub struct RolePolicy {
    overrides: HashMap<Role, HashSet<Permission>>,
}

impl RolePolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grant(&mut self, role: Role, perm: Permission) {
        self.overrides.entry(role).or_default().insert(perm);
    }

    pub fn revoke(&mut self, role: &Role, perm: &Permission) {
        if let Some(set) = self.overrides.get_mut(role) {
            set.remove(perm);
        }
    }

    pub fn permissions_for(&self, role: &Role) -> HashSet<Permission> {
        let mut base = default_permissions(role);
        if let Some(extra) = self.overrides.get(role) {
            base.extend(extra.iter().cloned());
        }
        base
    }
}
