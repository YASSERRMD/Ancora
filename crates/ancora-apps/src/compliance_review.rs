//! Compliance review application - government profile.
//!
//! Performs air-gapped policy evaluation against a local rule set.
//! No network access is ever required.

#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceProfile {
    Commercial,
    Government,
    Healthcare,
    Financial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ComplianceRule {
    pub id: String,
    pub description: String,
    pub severity: FindingSeverity,
    /// Substring that must be present in the artifact text to pass.
    pub required_keyword: Option<String>,
    /// Substring that must NOT be present in the artifact text to pass.
    pub forbidden_keyword: Option<String>,
}

impl ComplianceRule {
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        severity: FindingSeverity,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            severity,
            required_keyword: None,
            forbidden_keyword: None,
        }
    }

    pub fn require(mut self, keyword: impl Into<String>) -> Self {
        self.required_keyword = Some(keyword.into());
        self
    }

    pub fn forbid(mut self, keyword: impl Into<String>) -> Self {
        self.forbidden_keyword = Some(keyword.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub rule_id: String,
    pub severity: FindingSeverity,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub artifact_id: String,
    pub profile: ComplianceProfile,
    pub findings: Vec<Finding>,
    pub passed: bool,
}

impl ReviewResult {
    pub fn critical_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == FindingSeverity::Critical)
            .count()
    }

    pub fn high_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == FindingSeverity::High)
            .count()
    }
}

pub struct ComplianceReviewer {
    rules: Vec<ComplianceRule>,
    profile: ComplianceProfile,
}

impl ComplianceReviewer {
    pub fn new(profile: ComplianceProfile, rules: Vec<ComplianceRule>) -> Self {
        Self { rules, profile }
    }

    /// Government-profile preset: a curated minimal rule set.
    pub fn government_preset() -> Self {
        let rules = vec![
            ComplianceRule::new(
                "GOV-001",
                "Artifacts must include a data classification label.",
                FindingSeverity::Critical,
            )
            .require("CLASSIFICATION"),
            ComplianceRule::new(
                "GOV-002",
                "Artifacts must not contain plaintext secrets.",
                FindingSeverity::Critical,
            )
            .forbid("SECRET="),
            ComplianceRule::new(
                "GOV-003",
                "Artifacts must reference the authorising authority.",
                FindingSeverity::High,
            )
            .require("AUTHORITY"),
            ComplianceRule::new(
                "GOV-004",
                "Artifacts must include a retention schedule.",
                FindingSeverity::Medium,
            )
            .require("RETENTION"),
        ];
        Self::new(ComplianceProfile::Government, rules)
    }

    /// Evaluate an artifact text; returns a ReviewResult.
    pub fn review(&self, artifact_id: &str, text: &str) -> ReviewResult {
        let mut findings = Vec::new();

        for rule in &self.rules {
            if let Some(ref kw) = rule.required_keyword {
                if !text.contains(kw.as_str()) {
                    findings.push(Finding {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: format!("Rule {}: required keyword '{}' missing.", rule.id, kw),
                    });
                }
            }
            if let Some(ref kw) = rule.forbidden_keyword {
                if text.contains(kw.as_str()) {
                    findings.push(Finding {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        message: format!("Rule {}: forbidden keyword '{}' present.", rule.id, kw),
                    });
                }
            }
        }

        let passed = !findings.iter().any(|f| {
            f.severity == FindingSeverity::Critical || f.severity == FindingSeverity::High
        });

        ReviewResult {
            artifact_id: artifact_id.to_string(),
            profile: self.profile.clone(),
            findings,
            passed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn government_preset_fails_missing_classification() {
        let reviewer = ComplianceReviewer::government_preset();
        let result = reviewer.review("doc-001", "AUTHORITY: DoD\nRETENTION: 7 years");
        assert!(!result.passed);
        assert!(result.critical_count() > 0);
    }

    #[test]
    fn government_preset_passes_compliant_artifact() {
        let reviewer = ComplianceReviewer::government_preset();
        let text = "CLASSIFICATION: UNCLASSIFIED\nAUTHORITY: DoD\nRETENTION: 7 years";
        let result = reviewer.review("doc-002", text);
        assert!(result.passed);
    }
}
