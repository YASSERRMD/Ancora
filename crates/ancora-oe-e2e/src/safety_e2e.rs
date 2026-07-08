/// Safety monitor module: flags unsafe outputs based on keyword and pattern rules.

/// Severity of a safety flag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// A safety flag raised on an output.
#[derive(Debug, Clone)]
pub struct SafetyFlag {
    pub rule_id: String,
    pub description: String,
    pub severity: Severity,
    pub matched_text: String,
}

/// A safety rule that matches on a keyword.
#[derive(Debug, Clone)]
pub struct KeywordRule {
    pub rule_id: String,
    pub keyword: String,
    pub severity: Severity,
    pub description: String,
}

impl KeywordRule {
    pub fn new(
        rule_id: impl Into<String>,
        keyword: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            keyword: keyword.into(),
            severity,
            description: description.into(),
        }
    }

    pub fn check(&self, text: &str) -> Option<SafetyFlag> {
        if text.to_lowercase().contains(&self.keyword.to_lowercase()) {
            Some(SafetyFlag {
                rule_id: self.rule_id.clone(),
                description: self.description.clone(),
                severity: self.severity.clone(),
                matched_text: self.keyword.clone(),
            })
        } else {
            None
        }
    }
}

/// Safety monitor that applies a set of rules to output text.
#[derive(Debug, Default)]
pub struct SafetyMonitor {
    rules: Vec<KeywordRule>,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: KeywordRule) {
        self.rules.push(rule);
    }

    pub fn inspect(&self, text: &str) -> Vec<SafetyFlag> {
        self.rules.iter().filter_map(|r| r.check(text)).collect()
    }

    pub fn is_safe(&self, text: &str) -> bool {
        self.inspect(text).is_empty()
    }

    pub fn highest_severity(&self, text: &str) -> Option<Severity> {
        self.inspect(text).into_iter().map(|f| f.severity).max()
    }
}

/// Build a default safety monitor for testing.
pub fn default_safety_monitor() -> SafetyMonitor {
    let mut monitor = SafetyMonitor::new();
    monitor.add_rule(KeywordRule::new(
        "S001",
        "bomb",
        Severity::Critical,
        "Violence keyword",
    ));
    monitor.add_rule(KeywordRule::new(
        "S002",
        "hack",
        Severity::High,
        "Security keyword",
    ));
    monitor.add_rule(KeywordRule::new(
        "S003",
        "spam",
        Severity::Medium,
        "Spam keyword",
    ));
    monitor.add_rule(KeywordRule::new(
        "S004",
        "advertisement",
        Severity::Low,
        "Promotional content",
    ));
    monitor
}
