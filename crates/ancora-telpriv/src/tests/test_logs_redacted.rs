use crate::log_policy::{LogLevel, LogPolicy, LogRecord};

#[test]
fn log_email_field_redacted() {
    let policy = LogPolicy::default();
    let record = LogRecord {
        level: LogLevel::Info,
        message: "sign in".to_string(),
        fields: vec![
            ("email".to_string(), "bob@example.com".to_string()),
            ("action".to_string(), "login".to_string()),
        ],
    };
    let redacted = policy.apply(record).unwrap();
    let email_field = redacted.fields.iter().find(|(k, _)| k == "email").unwrap();
    assert_eq!(email_field.1, "[REDACTED]");
    // non-sensitive fields survive
    let action_field = redacted.fields.iter().find(|(k, _)| k == "action").unwrap();
    assert_eq!(action_field.1, "login");
}

#[test]
fn log_message_pii_scrubbed() {
    let policy = LogPolicy::default();
    let record = LogRecord {
        level: LogLevel::Warn,
        message: "failed login for user@example.org from 10.0.0.1".to_string(),
        fields: vec![],
    };
    let redacted = policy.apply(record).unwrap();
    assert!(!redacted.message.contains("user@example.org"));
    assert!(!redacted.message.contains("10.0.0.1"));
}

#[test]
fn debug_log_suppressed() {
    let policy = LogPolicy::default();
    let record = LogRecord {
        level: LogLevel::Debug,
        message: "verbose debug output".to_string(),
        fields: vec![],
    };
    assert!(policy.apply(record).is_none());
}

#[test]
fn error_log_passes_through() {
    let policy = LogPolicy::default();
    let record = LogRecord {
        level: LogLevel::Error,
        message: "disk full".to_string(),
        fields: vec![("error_code".to_string(), "ENOSPC".to_string())],
    };
    let redacted = policy.apply(record);
    assert!(redacted.is_some());
}
