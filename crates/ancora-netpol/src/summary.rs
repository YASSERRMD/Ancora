use crate::policy::NetworkPolicy;
use crate::audit::NetpolAuditLog;

#[derive(Debug)]
pub struct PolicySummary {
    pub tenant_id: String,
    pub total_rules: usize,
    pub allow_rules: usize,
    pub deny_rules: usize,
    pub default_posture: String,
    pub total_evaluations: usize,
    pub total_denied: usize,
    pub total_allowed: usize,
}

impl PolicySummary {
    pub fn from(policy: &NetworkPolicy, audit: &NetpolAuditLog) -> Self {
        let tenant_id = policy.tenant_id.clone();
        let total_evaluations = audit.all().filter(|r| r.tenant_id == policy.tenant_id).count();
        let total_denied = audit.denied_for(&tenant_id).len();
        let total_allowed = audit.allowed_for(&tenant_id).len();
        Self {
            total_rules: policy.rule_count(),
            allow_rules: policy.allow_rules().len(),
            deny_rules: policy.deny_rules().len(),
            default_posture: format!("{:?}", policy.default_posture),
            total_evaluations,
            total_denied,
            total_allowed,
            tenant_id,
        }
    }

    pub fn deny_rate(&self) -> f64 {
        if self.total_evaluations == 0 { 0.0 }
        else { self.total_denied as f64 / self.total_evaluations as f64 }
    }
}
