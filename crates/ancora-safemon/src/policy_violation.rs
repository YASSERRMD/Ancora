/// Policy violation detection for agent outputs.
///
/// Checks text against a configurable set of policy rules.
/// Rules can be keyword-based or pattern-based.

#[derive(Debug, Clone, PartialEq)]
pub enum ViolationKind {
    ConfidentialDataExposure,
    ProhibitedTopic,
    RestrictedInstruction,
    ComplianceRule(String),
}

#[derive(Debug, Clone)]
pub struct PolicyViolation {
    pub kind: ViolationKind,
    pub rule_id: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub id: String,
    pub kind: ViolationKind,
    pub keywords: Vec<String>,
    pub description: String,
}

impl PolicyRule {
    pub fn new(
        id: impl Into<String>,
        kind: ViolationKind,
        keywords: Vec<&str>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            keywords: keywords.into_iter().map(|s| s.to_lowercase()).collect(),
            description: description.into(),
        }
    }

    pub fn matches(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        self.keywords.iter().any(|kw| lower.contains(kw.as_str()))
    }
}

pub struct PolicyViolationDetector {
    rules: Vec<PolicyRule>,
}

impl PolicyViolationDetector {
    pub fn new() -> Self {
        let rules = vec![
            PolicyRule::new(
                "POL-001",
                ViolationKind::ConfidentialDataExposure,
                vec![
                    "confidential",
                    "proprietary",
                    "trade secret",
                    "internal only",
                ],
                "Confidential data exposure",
            ),
            PolicyRule::new(
                "POL-002",
                ViolationKind::ProhibitedTopic,
                vec!["how to make a bomb", "synthesize drugs", "make explosives"],
                "Prohibited topic - dangerous instructions",
            ),
            PolicyRule::new(
                "POL-003",
                ViolationKind::RestrictedInstruction,
                vec![
                    "bypass security",
                    "disable firewall",
                    "disable authentication",
                ],
                "Restricted instruction - security bypass",
            ),
            PolicyRule::new(
                "POL-004",
                ViolationKind::ComplianceRule("GDPR".to_string()),
                vec!["personal data", "data subject", "right to erasure"],
                "GDPR compliance keyword detected - verify context",
            ),
        ];
        Self { rules }
    }

    /// Returns the first policy violation found, or None.
    pub fn check(&self, text: &str) -> Option<PolicyViolation> {
        for rule in &self.rules {
            if rule.matches(text) {
                return Some(PolicyViolation {
                    kind: rule.kind.clone(),
                    rule_id: rule.id.clone(),
                    description: rule.description.clone(),
                });
            }
        }
        None
    }

    /// Returns all policy violations found.
    pub fn check_all(&self, text: &str) -> Vec<PolicyViolation> {
        self.rules
            .iter()
            .filter(|r| r.matches(text))
            .map(|r| PolicyViolation {
                kind: r.kind.clone(),
                rule_id: r.id.clone(),
                description: r.description.clone(),
            })
            .collect()
    }

    /// Add a custom rule at runtime.
    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }
}

impl Default for PolicyViolationDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_no_violation() {
        let d = PolicyViolationDetector::new();
        assert!(d.check("The sky is blue.").is_none());
    }

    #[test]
    fn confidential_triggers_violation() {
        let d = PolicyViolationDetector::new();
        let v = d.check("This document is confidential and for internal use.");
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.kind, ViolationKind::ConfidentialDataExposure);
        assert_eq!(v.rule_id, "POL-001");
    }

    #[test]
    fn restricted_instruction_detected() {
        let d = PolicyViolationDetector::new();
        let v = d.check("You should bypass security checks here.");
        assert!(v.is_some());
        assert_eq!(v.unwrap().kind, ViolationKind::RestrictedInstruction);
    }

    #[test]
    fn multiple_violations_found() {
        let d = PolicyViolationDetector::new();
        let vs = d.check_all("confidential data, bypass security");
        assert!(vs.len() >= 2);
    }
}
