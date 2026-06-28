/// Redaction policies for sensitive span attributes.
///
/// Prompt content and model responses may contain PII or secrets.
/// This module provides a policy-driven mechanism to redact or mask
/// attribute values before they are exported.

use crate::span::{AttributeValue, Span};

/// A named redaction policy controlling which attributes are masked.
#[derive(Debug, Clone, PartialEq)]
pub struct RedactPolicy {
    pub name: String,
    /// Attribute key prefixes whose values should be fully redacted.
    pub redact_keys: Vec<String>,
    /// Attribute keys whose string values should be truncated.
    pub truncate_keys: Vec<TruncateRule>,
}

/// Rule to truncate a string attribute to a maximum character count.
#[derive(Debug, Clone, PartialEq)]
pub struct TruncateRule {
    pub key_prefix: String,
    pub max_chars: usize,
}

/// Sentinel value placed where a redacted attribute once was.
pub const REDACTED_SENTINEL: &str = "<redacted>";

impl RedactPolicy {
    /// Policy that redacts nothing.
    pub fn passthrough() -> Self {
        RedactPolicy {
            name: "passthrough".into(),
            redact_keys: Vec::new(),
            truncate_keys: Vec::new(),
        }
    }

    /// Policy that redacts prompt and response content.
    pub fn redact_content() -> Self {
        RedactPolicy {
            name: "redact-content".into(),
            redact_keys: vec![
                "gen_ai.prompt".into(),
                "gen_ai.completion".into(),
                "gen_ai.request.messages".into(),
                "gen_ai.response.content".into(),
            ],
            truncate_keys: Vec::new(),
        }
    }

    /// Policy that truncates content to a short preview.
    pub fn truncate_content(max_chars: usize) -> Self {
        RedactPolicy {
            name: "truncate-content".into(),
            redact_keys: Vec::new(),
            truncate_keys: vec![
                TruncateRule {
                    key_prefix: "gen_ai.prompt".into(),
                    max_chars,
                },
                TruncateRule {
                    key_prefix: "gen_ai.completion".into(),
                    max_chars,
                },
            ],
        }
    }

    /// Returns true if the attribute key should be fully redacted.
    pub fn should_redact(&self, key: &str) -> bool {
        self.redact_keys.iter().any(|rk| key.starts_with(rk.as_str()))
    }

    /// Returns the truncation limit for the given key, if any.
    pub fn truncation_limit(&self, key: &str) -> Option<usize> {
        self.truncate_keys
            .iter()
            .find(|r| key.starts_with(r.key_prefix.as_str()))
            .map(|r| r.max_chars)
    }

    /// Apply this policy to a single attribute value in place.
    pub fn apply_to_value(&self, key: &str, value: &mut AttributeValue) {
        if self.should_redact(key) {
            *value = AttributeValue::String(REDACTED_SENTINEL.into());
            return;
        }
        if let Some(limit) = self.truncation_limit(key) {
            if let AttributeValue::String(ref s) = value.clone() {
                if s.len() > limit {
                    *value = AttributeValue::String(format!("{}...", &s[..limit]));
                }
            }
        }
    }

    /// Apply this policy to all attributes of a span, returning a cloned
    /// span with redacted / truncated fields.
    pub fn apply_to_span(&self, span: &Span) -> Span {
        let mut out = span.clone();
        for (k, v) in out.attributes.iter_mut() {
            self.apply_to_value(k, v);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::AttributeValue;

    #[test]
    fn passthrough_leaves_values() {
        let policy = RedactPolicy::passthrough();
        let mut val = AttributeValue::String("hello".into());
        policy.apply_to_value("gen_ai.prompt", &mut val);
        assert_eq!(val, AttributeValue::String("hello".into()));
    }

    #[test]
    fn redact_content_replaces_prompt() {
        let policy = RedactPolicy::redact_content();
        let mut val = AttributeValue::String("secret".into());
        policy.apply_to_value("gen_ai.prompt", &mut val);
        assert_eq!(val, AttributeValue::String(REDACTED_SENTINEL.into()));
    }

    #[test]
    fn truncate_policy_shortens_long_strings() {
        let policy = RedactPolicy::truncate_content(10);
        let mut val = AttributeValue::String("a very long prompt that exceeds the limit".into());
        policy.apply_to_value("gen_ai.prompt", &mut val);
        if let AttributeValue::String(s) = val {
            assert!(s.len() <= 13); // 10 + "..."
        }
    }
}
