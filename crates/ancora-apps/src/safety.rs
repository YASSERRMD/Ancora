/// Safety guardrails for sample applications.
///
/// All apps pass output through this layer before returning to callers.
/// Operates entirely offline; no remote policy service required.

#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailOutcome {
    Allow,
    Block { reason: String },
    Redact { redacted: String },
}

#[derive(Debug, Clone)]
pub struct GuardrailRule {
    pub id: String,
    pub pattern: String,
    pub action: GuardrailAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailAction {
    Block,
    Redact { replacement: String },
}

impl GuardrailRule {
    pub fn block(id: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            action: GuardrailAction::Block,
        }
    }

    pub fn redact(
        id: impl Into<String>,
        pattern: impl Into<String>,
        replacement: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            pattern: pattern.into(),
            action: GuardrailAction::Redact {
                replacement: replacement.into(),
            },
        }
    }
}

pub struct SafetyGuardrail {
    rules: Vec<GuardrailRule>,
}

impl SafetyGuardrail {
    pub fn new(rules: Vec<GuardrailRule>) -> Self {
        Self { rules }
    }

    /// Minimal default set suitable for all sample apps.
    pub fn default_rules() -> Self {
        Self::new(vec![
            GuardrailRule::block("GR-001", "rm -rf"),
            GuardrailRule::block("GR-002", "DROP TABLE"),
            GuardrailRule::redact(
                "GR-003",
                "SECRET=",
                "[REDACTED]",
            ),
            GuardrailRule::block("GR-004", "<script>"),
        ])
    }

    /// Evaluate output text and return the guardrail outcome.
    pub fn evaluate(&self, text: &str) -> GuardrailOutcome {
        let mut current = text.to_string();

        for rule in &self.rules {
            if current.contains(rule.pattern.as_str()) {
                match &rule.action {
                    GuardrailAction::Block => {
                        return GuardrailOutcome::Block {
                            reason: format!("rule {} triggered on pattern '{}'", rule.id, rule.pattern),
                        };
                    }
                    GuardrailAction::Redact { replacement } => {
                        current = current.replace(rule.pattern.as_str(), replacement.as_str());
                    }
                }
            }
        }

        if current == text {
            GuardrailOutcome::Allow
        } else {
            GuardrailOutcome::Redact { redacted: current }
        }
    }

    pub fn is_safe(&self, text: &str) -> bool {
        !matches!(self.evaluate(text), GuardrailOutcome::Block { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_dangerous_command() {
        let guard = SafetyGuardrail::default_rules();
        let outcome = guard.evaluate("please run rm -rf /");
        assert!(matches!(outcome, GuardrailOutcome::Block { .. }));
    }

    #[test]
    fn allows_clean_text() {
        let guard = SafetyGuardrail::default_rules();
        let outcome = guard.evaluate("Here is a summary of the document.");
        assert_eq!(outcome, GuardrailOutcome::Allow);
    }

    #[test]
    fn redacts_secret() {
        let guard = SafetyGuardrail::default_rules();
        let outcome = guard.evaluate("config SECRET=abc123 value");
        assert!(matches!(outcome, GuardrailOutcome::Redact { .. }));
    }
}
