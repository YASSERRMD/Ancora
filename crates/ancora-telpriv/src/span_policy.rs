/// Span-level redaction policy.
///
/// Determines which span attributes are safe to export and which must be
/// scrubbed or dropped before leaving the process boundary.
use crate::classification::DataClass;

/// Action to take for a given span attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanAction {
    /// Export the attribute value as-is.
    Allow,
    /// Replace the value with a fixed redaction marker.
    Redact,
    /// Drop the attribute entirely.
    Drop,
}

/// Policy configuration for span attribute handling.
#[derive(Debug, Clone)]
pub struct SpanPolicy {
    /// Minimum data class that triggers automatic redaction.
    pub redact_at_or_above: DataClass,
    /// Always-drop attribute name prefixes (e.g. "prompt.", "user.").
    pub drop_prefixes: Vec<String>,
}

impl Default for SpanPolicy {
    fn default() -> Self {
        SpanPolicy {
            redact_at_or_above: DataClass::Sensitive,
            drop_prefixes: vec!["prompt.".to_string(), "user.content.".to_string()],
        }
    }
}

impl SpanPolicy {
    /// Decide what to do with the named attribute given a data class.
    pub fn evaluate(&self, attr_name: &str, class: DataClass) -> SpanAction {
        // Check drop prefixes first - highest priority.
        for prefix in &self.drop_prefixes {
            if attr_name.starts_with(prefix.as_str()) {
                return SpanAction::Drop;
            }
        }
        // Then check classification threshold.
        if class >= self.redact_at_or_above {
            return SpanAction::Redact;
        }
        SpanAction::Allow
    }
}

/// Apply the policy to a collection of (name, value, class) triples.
/// Returns only the attributes that survive (with values potentially redacted).
pub fn apply_span_policy(
    policy: &SpanPolicy,
    attrs: &[(&str, &str, DataClass)],
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (name, value, class) in attrs {
        match policy.evaluate(name, class.clone()) {
            SpanAction::Allow => out.push((name.to_string(), value.to_string())),
            SpanAction::Redact => out.push((name.to_string(), "[REDACTED]".to_string())),
            SpanAction::Drop => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_public_attr() {
        let policy = SpanPolicy::default();
        assert_eq!(
            policy.evaluate("span.name", DataClass::Public),
            SpanAction::Allow
        );
    }

    #[test]
    fn redact_sensitive_attr() {
        let policy = SpanPolicy::default();
        assert_eq!(
            policy.evaluate("token.count", DataClass::Sensitive),
            SpanAction::Redact
        );
    }

    #[test]
    fn drop_prompt_prefix() {
        let policy = SpanPolicy::default();
        assert_eq!(
            policy.evaluate("prompt.text", DataClass::Public),
            SpanAction::Drop
        );
    }
}
