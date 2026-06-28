pub mod assignment;
pub mod audit;
pub mod bulk;
pub mod guard;
pub mod summary;
pub mod checker;
pub mod permission;
pub mod policy;
pub mod role;

pub use assignment::{AssignmentStore, RoleAssignment};
pub use audit::{RbacAuditLog, RbacEvent};
pub use checker::{AuthzDecision, PermissionChecker};
pub use guard::RbacGuard;
pub use permission::Permission;
pub use policy::{default_permissions, RolePolicy};
pub use role::Role;

#[cfg(test)]
mod tests {
    mod test_role_precedence;
    mod test_permissions_viewer;
    mod test_permissions_developer;
    mod test_permissions_operator;
    mod test_permissions_admin;
    mod test_assignment_store;
    mod test_permission_checker;
    mod test_role_dominance;
    mod test_policy_override;
    mod test_revoke_assignment;
    mod test_subjects_with_role;
    mod test_cross_tenant;
    mod test_authz_deny_reasons;
    mod test_all_roles;
    mod test_admin_inherits_all;
    mod test_viewer_cannot_write;
    mod test_developer_can_execute;
    mod test_operator_can_secret;
    mod test_default_policy_immutable;
    mod test_role_as_str;
    mod test_rbac_guard;
    mod test_rbac_audit;
    mod test_bulk_ops;
    mod test_summary;
    mod test_permission_strings;
}
