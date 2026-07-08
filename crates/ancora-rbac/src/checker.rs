use crate::assignment::AssignmentStore;
use crate::permission::Permission;
use crate::policy::RolePolicy;

#[derive(Debug, PartialEq, Eq)]
pub enum AuthzDecision {
    Allow,
    Deny(String),
}

pub struct PermissionChecker<'a> {
    assignments: &'a AssignmentStore,
    policy: &'a RolePolicy,
}

impl<'a> PermissionChecker<'a> {
    pub fn new(assignments: &'a AssignmentStore, policy: &'a RolePolicy) -> Self {
        Self {
            assignments,
            policy,
        }
    }

    pub fn check(&self, subject: &str, tenant_id: &str, perm: &Permission) -> AuthzDecision {
        match self.assignments.role_of(subject, tenant_id) {
            None => AuthzDecision::Deny(format!("no role assigned for {subject} in {tenant_id}")),
            Some(role) => {
                let perms = self.policy.permissions_for(role);
                if perms.contains(perm) {
                    AuthzDecision::Allow
                } else {
                    AuthzDecision::Deny(format!(
                        "{subject} has role {} which lacks {}",
                        role.as_str(),
                        perm.as_str()
                    ))
                }
            }
        }
    }

    pub fn is_allowed(&self, subject: &str, tenant_id: &str, perm: &Permission) -> bool {
        self.check(subject, tenant_id, perm) == AuthzDecision::Allow
    }

    pub fn require_role_dominates(
        &self,
        subject: &str,
        tenant_id: &str,
        minimum: &crate::role::Role,
    ) -> AuthzDecision {
        match self.assignments.role_of(subject, tenant_id) {
            None => AuthzDecision::Deny(format!("no role for {subject}")),
            Some(role) => {
                if role.dominates(minimum) {
                    AuthzDecision::Allow
                } else {
                    AuthzDecision::Deny(format!(
                        "{subject} has role {} which is below minimum {}",
                        role.as_str(),
                        minimum.as_str()
                    ))
                }
            }
        }
    }
}
