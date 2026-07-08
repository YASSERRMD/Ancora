use crate::policy::NetworkPolicy;
use crate::rule::Effect;

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub rule_id: String,
    pub kind: IssueKind,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueKind {
    DuplicateId,
    ShadowedRule,
    NoRules,
}

pub struct PolicyValidator;

impl PolicyValidator {
    pub fn validate(policy: &NetworkPolicy) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        if policy.rules.is_empty() {
            issues.push(ValidationIssue {
                rule_id: String::new(),
                kind: IssueKind::NoRules,
                description: "policy has no rules, default posture applies to all traffic"
                    .to_string(),
            });
            return issues;
        }
        let mut seen_ids = std::collections::HashSet::new();
        for rule in &policy.rules {
            if !seen_ids.insert(&rule.id) {
                issues.push(ValidationIssue {
                    rule_id: rule.id.clone(),
                    kind: IssueKind::DuplicateId,
                    description: format!("duplicate rule id '{}'", rule.id),
                });
            }
        }
        for i in 0..policy.rules.len() {
            for j in (i + 1)..policy.rules.len() {
                let earlier = &policy.rules[i];
                let later = &policy.rules[j];
                if earlier.effect != later.effect
                    && earlier.host_pattern == "*"
                    && earlier.port.is_none()
                    && later.priority > earlier.priority
                    && matches!(earlier.effect, Effect::Deny)
                {
                    issues.push(ValidationIssue {
                        rule_id: later.id.clone(),
                        kind: IssueKind::ShadowedRule,
                        description: format!(
                            "rule '{}' may be shadowed by wildcard deny rule '{}' at priority {}",
                            later.id, earlier.id, earlier.priority
                        ),
                    });
                }
            }
        }
        issues
    }

    pub fn is_valid(policy: &NetworkPolicy) -> bool {
        Self::validate(policy).is_empty()
    }
}
