// Security: output filter -- redact sensitive fields before returning to caller.

use std::collections::BTreeMap;

fn redact(value: &str, fields: &[&str], json_map: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let _ = value;
    let mut out = json_map.clone();
    for &field in fields {
        if out.contains_key(field) {
            out.insert(field.to_string(), "[REDACTED]".to_string());
        }
    }
    out
}

const REDACT_FIELDS: &[&str] = &["api_key", "password", "secret", "token"];

#[test]
fn test_non_sensitive_field_passes_through() {
    let mut map = BTreeMap::new();
    map.insert("model".to_string(), "claude-3".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["model"], "claude-3");
}

#[test]
fn test_api_key_field_redacted() {
    let mut map = BTreeMap::new();
    map.insert("api_key".to_string(), "sk-abc123".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["api_key"], "[REDACTED]");
}

#[test]
fn test_password_field_redacted() {
    let mut map = BTreeMap::new();
    map.insert("password".to_string(), "hunter2".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["password"], "[REDACTED]");
}

#[test]
fn test_multiple_sensitive_fields_all_redacted() {
    let mut map = BTreeMap::new();
    map.insert("api_key".to_string(), "key".to_string());
    map.insert("token".to_string(), "tok".to_string());
    map.insert("user".to_string(), "alice".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["api_key"], "[REDACTED]");
    assert_eq!(out["token"], "[REDACTED]");
    assert_eq!(out["user"], "alice");
}

#[test]
fn test_field_not_in_map_left_alone() {
    let map = BTreeMap::new();
    let out = redact("", REDACT_FIELDS, &map);
    assert!(out.is_empty());
}

#[test]
fn test_secret_field_redacted() {
    let mut map = BTreeMap::new();
    map.insert("secret".to_string(), "mysecret".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["secret"], "[REDACTED]");
}

#[test]
fn test_redacted_value_is_literal_string() {
    let mut map = BTreeMap::new();
    map.insert("password".to_string(), "anything".to_string());
    let out = redact("", REDACT_FIELDS, &map);
    assert_eq!(out["password"], "[REDACTED]");
}
