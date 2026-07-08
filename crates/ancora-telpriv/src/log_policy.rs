/// Log-level redaction policy.
///
/// Log records may contain free-form strings that inadvertently include PII.
/// This module defines rules for scrubbing log messages before export.
use crate::pii_scrub::scrub_pii;

/// Severity of a log record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// A log record ready for evaluation.
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub level: LogLevel,
    pub message: String,
    /// Structured fields attached to the record.
    pub fields: Vec<(String, String)>,
}

/// Configuration for the log redaction policy.
#[derive(Debug, Clone)]
pub struct LogPolicy {
    /// Minimum log level that is exported at all.
    pub min_export_level: LogLevel,
    /// Whether to run PII scrubbing on the message body.
    pub scrub_message: bool,
    /// Field names whose values must always be redacted.
    pub sensitive_fields: Vec<String>,
}

impl Default for LogPolicy {
    fn default() -> Self {
        LogPolicy {
            min_export_level: LogLevel::Info,
            scrub_message: true,
            sensitive_fields: vec![
                "user_id".to_string(),
                "email".to_string(),
                "ip".to_string(),
                "password".to_string(),
            ],
        }
    }
}

/// Result of applying the policy to a log record.
#[derive(Debug, Clone)]
pub struct RedactedLog {
    pub level: LogLevel,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

impl LogPolicy {
    /// Apply the policy to a log record. Returns None if the record is suppressed.
    pub fn apply(&self, record: LogRecord) -> Option<RedactedLog> {
        if record.level < self.min_export_level {
            return None;
        }

        let message = if self.scrub_message {
            scrub_pii(&record.message)
        } else {
            record.message
        };

        let fields = record
            .fields
            .into_iter()
            .map(|(k, v)| {
                if self.sensitive_fields.iter().any(|s| s == &k) {
                    (k, "[REDACTED]".to_string())
                } else {
                    (k, v)
                }
            })
            .collect();

        Some(RedactedLog {
            level: record.level,
            message,
            fields,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_suppressed() {
        let policy = LogPolicy::default();
        let record = LogRecord {
            level: LogLevel::Debug,
            message: "debug info".to_string(),
            fields: vec![],
        };
        assert!(policy.apply(record).is_none());
    }

    #[test]
    fn sensitive_field_redacted() {
        let policy = LogPolicy::default();
        let record = LogRecord {
            level: LogLevel::Info,
            message: "user logged in".to_string(),
            fields: vec![("email".to_string(), "alice@example.com".to_string())],
        };
        let redacted = policy.apply(record).unwrap();
        assert_eq!(redacted.fields[0].1, "[REDACTED]");
    }
}
