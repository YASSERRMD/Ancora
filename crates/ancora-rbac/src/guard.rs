use crate::assignment::AssignmentStore;
use crate::permission::Permission;
use crate::policy::RolePolicy;
use crate::role::Role;

pub struct RbacGuard<'a> {
    assignments: &'a AssignmentStore,
    policy: &'a RolePolicy,
}

impl<'a> RbacGuard<'a> {
    pub fn new(assignments: &'a AssignmentStore, policy: &'a RolePolicy) -> Self {
        Self {
            assignments,
            policy,
        }
    }

    pub fn assert_permission(
        &self,
        subject: &str,
        tenant_id: &str,
        perm: &Permission,
    ) -> Result<(), String> {
        let role = self
            .assignments
            .role_of(subject, tenant_id)
            .ok_or_else(|| format!("no role for {subject} in {tenant_id}"))?;
        let perms = self.policy.permissions_for(role);
        if perms.contains(perm) {
            Ok(())
        } else {
            Err(format!("{subject} lacks {}", perm.as_str()))
        }
    }

    pub fn assert_minimum_role(
        &self,
        subject: &str,
        tenant_id: &str,
        minimum: &Role,
    ) -> Result<(), String> {
        let role = self
            .assignments
            .role_of(subject, tenant_id)
            .ok_or_else(|| format!("no role for {subject}"))?;
        if role.dominates(minimum) {
            Ok(())
        } else {
            Err(format!(
                "{subject} role {} below {}",
                role.as_str(),
                minimum.as_str()
            ))
        }
    }
}
