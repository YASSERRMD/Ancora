//! Data classification levels for telemetry fields.
//!
//! Used by policies to decide whether an attribute may be exported,
//! must be redacted, or must be dropped entirely.

/// The classification level of a piece of data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataClass {
    /// Freely shareable; no restrictions.
    Public,
    /// Internal use only; may be aggregated but not shown to third parties.
    Internal,
    /// Contains PII or confidential business data; must be redacted in telemetry.
    Sensitive,
    /// Highly sensitive (credentials, keys, health data); must be dropped.
    Critical,
}

impl DataClass {
    /// Parse a string label (case-insensitive) into a DataClass.
    pub fn from_label(label: &str) -> Option<DataClass> {
        match label.to_ascii_lowercase().as_str() {
            "public" => Some(DataClass::Public),
            "internal" => Some(DataClass::Internal),
            "sensitive" => Some(DataClass::Sensitive),
            "critical" => Some(DataClass::Critical),
            _ => None,
        }
    }

    /// Returns true if this class requires at least redaction.
    pub fn requires_redaction(&self) -> bool {
        self >= &DataClass::Sensitive
    }

    /// Returns true if this class requires the field to be dropped entirely.
    pub fn requires_drop(&self) -> bool {
        self == &DataClass::Critical
    }

    /// String label for this class.
    pub fn as_label(&self) -> &'static str {
        match self {
            DataClass::Public => "public",
            DataClass::Internal => "internal",
            DataClass::Sensitive => "sensitive",
            DataClass::Critical => "critical",
        }
    }
}

/// Classify an attribute name heuristically based on known patterns.
pub fn classify_attr(name: &str) -> DataClass {
    let lower = name.to_ascii_lowercase();
    if lower.contains("password")
        || lower.contains("secret")
        || lower.contains("key")
        || lower.contains("token")
        || lower.contains("credential")
    {
        return DataClass::Critical;
    }
    if lower.contains("email")
        || lower.contains("phone")
        || lower.contains("ssn")
        || lower.contains("user_id")
        || lower.contains("ip_addr")
    {
        return DataClass::Sensitive;
    }
    if lower.starts_with("internal.") || lower.starts_with("_") {
        return DataClass::Internal;
    }
    DataClass::Public
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(DataClass::Public < DataClass::Internal);
        assert!(DataClass::Internal < DataClass::Sensitive);
        assert!(DataClass::Sensitive < DataClass::Critical);
    }

    #[test]
    fn classify_secret() {
        assert_eq!(classify_attr("api_secret"), DataClass::Critical);
    }

    #[test]
    fn classify_email() {
        assert_eq!(classify_attr("user_email"), DataClass::Sensitive);
    }

    #[test]
    fn classify_span_name() {
        assert_eq!(classify_attr("span.name"), DataClass::Public);
    }
}
