use crate::label::SensitivityLevel;
use crate::policy::ClassificationPolicy;
use crate::record::DataRecord;

#[derive(Debug, PartialEq, Eq)]
pub enum EnforcementDecision {
    Allow,
    Deny(String),
}

pub struct ClassificationEnforcer;

impl ClassificationEnforcer {
    pub fn check_write(policy: &ClassificationPolicy, record: &DataRecord) -> EnforcementDecision {
        if record.level.is_above(&policy.max_allowed_level) {
            return EnforcementDecision::Deny(format!(
                "record level {} exceeds policy maximum {}",
                record.level, policy.max_allowed_level
            ));
        }
        if policy.deny_public_write && record.level == SensitivityLevel::Public {
            return EnforcementDecision::Deny(
                "write to PUBLIC classification is not allowed by policy".to_string(),
            );
        }
        if policy.require_category_tag && record.tags.is_empty() {
            return EnforcementDecision::Deny(
                "policy requires at least one category tag".to_string(),
            );
        }
        EnforcementDecision::Allow
    }

    pub fn check_read(
        policy: &ClassificationPolicy,
        record: &DataRecord,
        clearance: &SensitivityLevel,
    ) -> EnforcementDecision {
        if !clearance.is_at_least(&record.level) {
            return EnforcementDecision::Deny(format!(
                "clearance {} is insufficient for record level {}",
                clearance, record.level
            ));
        }
        if record.level.is_above(&policy.max_allowed_level) {
            return EnforcementDecision::Deny(format!(
                "record level {} is above tenant policy ceiling {}",
                record.level, policy.max_allowed_level
            ));
        }
        EnforcementDecision::Allow
    }

    pub fn is_allowed(decision: &EnforcementDecision) -> bool {
        decision == &EnforcementDecision::Allow
    }
}
