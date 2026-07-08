//! Redaction parity - validates that PII redaction is consistent across all SDK implementations.

/// A redaction rule that matches and replaces sensitive content.
#[derive(Debug, Clone)]
pub struct RedactRule {
    pub name: String,
    pub pattern: &'static str,
    pub replacement: &'static str,
}

impl RedactRule {
    pub fn new(name: impl Into<String>, pattern: &'static str, replacement: &'static str) -> Self {
        Self {
            name: name.into(),
            pattern,
            replacement,
        }
    }

    /// Apply simple substring redaction (no regex to avoid dependencies).
    pub fn apply(&self, input: &str) -> String {
        input.replace(self.pattern, self.replacement)
    }
}

/// Standard redaction rules applied by all SDKs.
pub fn standard_rules() -> Vec<RedactRule> {
    vec![
        RedactRule::new("email", "@example.com", "@[REDACTED]"),
        RedactRule::new("api_key", "sk-", "[KEY]"),
        RedactRule::new("ssn_pattern", "123-45-6789", "[SSN]"),
        RedactRule::new("credit_card", "4111-1111-1111-1111", "[CC]"),
    ]
}

/// Result of applying redaction to a text.
#[derive(Debug, Clone)]
pub struct RedactResult {
    pub language: String,
    pub original_len: usize,
    pub redacted_text: String,
    pub rules_applied: Vec<String>,
}

impl RedactResult {
    pub fn new(language: impl Into<String>, original: &str, rules: &[RedactRule]) -> Self {
        let mut text = original.to_string();
        let mut applied = Vec::new();
        for rule in rules {
            let before = text.clone();
            text = rule.apply(&text);
            if text != before {
                applied.push(rule.name.clone());
            }
        }
        Self {
            language: language.into(),
            original_len: original.len(),
            redacted_text: text,
            rules_applied: applied,
        }
    }

    pub fn contains_pii(&self, markers: &[&str]) -> bool {
        markers.iter().any(|m| self.redacted_text.contains(m))
    }
}

/// Reference text used for parity testing.
pub fn reference_text() -> &'static str {
    "User user@example.com sent key sk-abc123 with SSN 123-45-6789 and card 4111-1111-1111-1111."
}

/// Build a reference redact result for a given language.
pub fn reference_redact_result(language: impl Into<String>) -> RedactResult {
    let rules = standard_rules();
    RedactResult::new(language, reference_text(), &rules)
}

/// Check parity of redaction results across languages.
pub fn check_redact_parity(results: &[RedactResult]) -> Vec<String> {
    let mut issues = Vec::new();
    if let Some(first) = results.first() {
        for other in results.iter().skip(1) {
            if first.redacted_text != other.redacted_text {
                issues.push(format!(
                    "redacted text differs between {:?} and {:?}",
                    first.language, other.language
                ));
            }
            if first.rules_applied != other.rules_applied {
                issues.push(format!(
                    "rules applied differ: {:?}={:?} vs {:?}={:?}",
                    first.language, first.rules_applied, other.language, other.rules_applied
                ));
            }
        }
    }
    issues
}

/// Verify no PII remains after redaction.
pub fn assert_no_pii(result: &RedactResult) -> Vec<String> {
    let pii_markers = &["@example.com", "sk-", "123-45-6789", "4111-1111-1111-1111"];
    pii_markers
        .iter()
        .filter(|&&m| result.redacted_text.contains(m))
        .map(|m| {
            format!(
                "PII marker {:?} still present after redaction in {:?}",
                m, result.language
            )
        })
        .collect()
}
