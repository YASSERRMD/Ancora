//! Telemetry privacy controls: redaction and filtering of sensitive data.

/// Strategy for handling a field that may contain PII.
#[derive(Debug, Clone, PartialEq)]
pub enum PrivacyStrategy {
    /// Emit the field as-is.
    Allow,
    /// Replace the field value with a fixed redaction marker.
    Redact,
    /// Drop the field entirely from the telemetry record.
    Drop,
}

/// Describes how a named attribute should be treated for privacy.
#[derive(Debug, Clone)]
pub struct FieldPolicy {
    pub field_name: String,
    pub strategy: PrivacyStrategy,
}

impl FieldPolicy {
    pub fn allow(field_name: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            strategy: PrivacyStrategy::Allow,
        }
    }

    pub fn redact(field_name: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            strategy: PrivacyStrategy::Redact,
        }
    }

    pub fn drop(field_name: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            strategy: PrivacyStrategy::Drop,
        }
    }
}

/// Applies privacy policies to a map of attributes.
pub struct PrivacyFilter {
    policies: Vec<FieldPolicy>,
    default_strategy: PrivacyStrategy,
}

impl PrivacyFilter {
    pub fn new(policies: Vec<FieldPolicy>, default_strategy: PrivacyStrategy) -> Self {
        Self {
            policies,
            default_strategy,
        }
    }

    /// Apply the filter to a list of (key, value) pairs.
    pub fn apply(&self, attrs: Vec<(String, String)>) -> Vec<(String, String)> {
        attrs
            .into_iter()
            .filter_map(|(k, v)| {
                let strategy = self
                    .policies
                    .iter()
                    .find(|p| p.field_name == k)
                    .map(|p| &p.strategy)
                    .unwrap_or(&self.default_strategy);
                match strategy {
                    PrivacyStrategy::Allow => Some((k, v)),
                    PrivacyStrategy::Redact => Some((k, "[REDACTED]".to_string())),
                    PrivacyStrategy::Drop => None,
                }
            })
            .collect()
    }
}
