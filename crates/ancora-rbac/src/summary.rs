use crate::assignment::AssignmentStore;
use crate::policy::{default_permissions, RolePolicy};

pub struct SubjectSummary {
    pub subject: String,
    pub tenant_id: String,
    pub role: Option<String>,
    pub permission_count: usize,
}

pub fn summarize(
    store: &AssignmentStore,
    policy: &RolePolicy,
    subject: &str,
    tenant_id: &str,
) -> SubjectSummary {
    let role = store.role_of(subject, tenant_id);
    let permission_count = role.map(|r| policy.permissions_for(r).len()).unwrap_or(0);
    SubjectSummary {
        subject: subject.to_string(),
        tenant_id: tenant_id.to_string(),
        role: role.map(|r| r.as_str().to_string()),
        permission_count,
    }
}
