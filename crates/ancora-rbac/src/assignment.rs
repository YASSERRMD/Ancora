use crate::role::Role;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RoleAssignment {
    pub subject: String,
    pub tenant_id: String,
    pub role: Role,
    pub granted_at_tick: u64,
}

impl RoleAssignment {
    pub fn new(
        subject: impl Into<String>,
        tenant_id: impl Into<String>,
        role: Role,
        granted_at_tick: u64,
    ) -> Self {
        Self {
            subject: subject.into(),
            tenant_id: tenant_id.into(),
            role,
            granted_at_tick,
        }
    }
}

#[derive(Debug, Default)]
pub struct AssignmentStore {
    assignments: HashMap<(String, String), RoleAssignment>,
}

impl AssignmentStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign(&mut self, assignment: RoleAssignment) {
        let key = (assignment.subject.clone(), assignment.tenant_id.clone());
        self.assignments.insert(key, assignment);
    }

    pub fn get(&self, subject: &str, tenant_id: &str) -> Option<&RoleAssignment> {
        self.assignments
            .get(&(subject.to_string(), tenant_id.to_string()))
    }

    pub fn revoke(&mut self, subject: &str, tenant_id: &str) -> bool {
        self.assignments
            .remove(&(subject.to_string(), tenant_id.to_string()))
            .is_some()
    }

    pub fn role_of(&self, subject: &str, tenant_id: &str) -> Option<&Role> {
        self.get(subject, tenant_id).map(|a| &a.role)
    }

    pub fn subjects_with_role<'a>(&'a self, role: &'a Role, tenant_id: &'a str) -> Vec<&'a str> {
        self.assignments
            .values()
            .filter(|a| &a.role == role && a.tenant_id == tenant_id)
            .map(|a| a.subject.as_str())
            .collect()
    }
}
