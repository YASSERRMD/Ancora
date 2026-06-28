use crate::assignment::{AssignmentStore, RoleAssignment};
use crate::checker::AuthzDecision;
use crate::permission::Permission;
use crate::policy::RolePolicy;
use crate::role::Role;

pub fn assign_bulk(store: &mut AssignmentStore, assignments: Vec<RoleAssignment>) {
    for a in assignments { store.assign(a); }
}

pub fn check_all(
    store: &AssignmentStore,
    policy: &RolePolicy,
    subject: &str,
    tenant_id: &str,
    perms: &[Permission],
) -> Vec<AuthzDecision> {
    let checker = crate::checker::PermissionChecker::new(store, policy);
    perms.iter().map(|p| checker.check(subject, tenant_id, p)).collect()
}

pub fn effective_role_set(role: &Role) -> Vec<Role> {
    Role::all().into_iter().filter(|r| role.dominates(r)).collect()
}
