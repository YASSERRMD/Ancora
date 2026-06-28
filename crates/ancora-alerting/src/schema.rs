use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

/// An alert rule definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub severity: Severity,
    pub description: String,
    pub runbook_url: String,
    pub labels: std::collections::HashMap<String, String>,
}

impl AlertRule {
    pub fn new(
        name: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
        runbook_url: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            severity,
            description: description.into(),
            runbook_url: runbook_url.into(),
            labels: Default::default(),
        }
    }

    pub fn has_runbook(&self) -> bool {
        !self.runbook_url.is_empty()
    }
}

/// A fired alert instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiredAlert {
    pub rule_name: String,
    pub severity: Severity,
    pub message: String,
    pub runbook_url: String,
    pub labels: std::collections::HashMap<String, String>,
    pub fired_at_secs: u64,
    pub fingerprint: String,
}

impl FiredAlert {
    pub fn from_rule(rule: &AlertRule, message: impl Into<String>, at: u64) -> Self {
        let msg: String = message.into();
        let fingerprint = format!("{}:{}", rule.name, &msg[..msg.len().min(32)]);
        Self {
            rule_name: rule.name.clone(),
            severity: rule.severity.clone(),
            message: msg,
            runbook_url: rule.runbook_url.clone(),
            labels: rule.labels.clone(),
            fired_at_secs: at,
            fingerprint,
        }
    }
}

/// Compute a deterministic fingerprint for a (rule, message) pair.
pub fn fingerprint(rule_name: &str, msg: &str) -> String {
    format!("{}:{}", rule_name, &msg[..msg.len().min(32)])
}
