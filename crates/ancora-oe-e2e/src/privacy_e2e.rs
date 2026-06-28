/// Privacy and telemetry redaction module.
/// Ensures sensitive data (PII, secrets) is scrubbed before export.

use std::collections::HashMap;

/// A pattern to redact from strings.
#[derive(Debug, Clone)]
pub struct RedactionPattern {
    pub label: String,
    pub placeholder: String,
    /// Simple substring to find and replace. Real code would use regex.
    pub find: String,
}

impl RedactionPattern {
    pub fn new(label: impl Into<String>, find: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            placeholder: placeholder.into(),
            find: find.into(),
        }
    }

    pub fn redact(&self, text: &str) -> String {
        text.replace(&self.find, &self.placeholder)
    }

    pub fn matches(&self, text: &str) -> bool {
        text.contains(&self.find)
    }
}

/// Redactor that applies multiple patterns in order.
#[derive(Debug, Default)]
pub struct Redactor {
    patterns: Vec<RedactionPattern>,
}

impl Redactor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pattern(&mut self, pattern: RedactionPattern) {
        self.patterns.push(pattern);
    }

    pub fn redact(&self, text: &str) -> String {
        let mut out = text.to_string();
        for p in &self.patterns {
            out = p.redact(&out);
        }
        out
    }

    pub fn has_sensitive_data(&self, text: &str) -> bool {
        self.patterns.iter().any(|p| p.matches(text))
    }

    pub fn redact_map(&self, attrs: &HashMap<String, String>) -> HashMap<String, String> {
        attrs
            .iter()
            .map(|(k, v)| (k.clone(), self.redact(v)))
            .collect()
    }
}

/// Build a default redactor for testing.
pub fn default_redactor() -> Redactor {
    let mut r = Redactor::new();
    r.add_pattern(RedactionPattern::new("email", "user@example.com", "[REDACTED_EMAIL]"));
    r.add_pattern(RedactionPattern::new("phone", "+1-555-0100", "[REDACTED_PHONE]"));
    r.add_pattern(RedactionPattern::new("api_key", "sk-secret-12345", "[REDACTED_KEY]"));
    r.add_pattern(RedactionPattern::new("ssn", "123-45-6789", "[REDACTED_SSN]"));
    r
}

/// Verify that the redacted output contains no sensitive data.
pub fn assert_no_sensitive_data(redactor: &Redactor, text: &str) -> Result<(), Vec<String>> {
    let redacted = redactor.redact(text);
    let leaks: Vec<String> = redactor
        .patterns
        .iter()
        .filter(|p| redacted.contains(&p.find))
        .map(|p| format!("Pattern '{}' still present after redaction", p.label))
        .collect();
    if leaks.is_empty() {
        Ok(())
    } else {
        Err(leaks)
    }
}
