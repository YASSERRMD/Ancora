use crate::hashing::{hash_value, hash_with_salt, CorrelationToken};

#[test]
fn same_input_same_hash() {
    assert_eq!(hash_value("user-999"), hash_value("user-999"));
}

#[test]
fn different_users_different_tokens() {
    let t1 = CorrelationToken::from_raw("user-001", "salt");
    let t2 = CorrelationToken::from_raw("user-002", "salt");
    assert_ne!(t1, t2);
}

#[test]
fn correlation_across_records() {
    let salt = "deploy-salt";
    let session_id = "session-xyz";

    // Two telemetry records from the same session should share the same token.
    let token_a = CorrelationToken::from_raw(session_id, salt);
    let token_b = CorrelationToken::from_raw(session_id, salt);
    assert_eq!(token_a, token_b, "same session produces same token");

    // The raw value is not recoverable.
    assert!(!token_a.as_str().contains(session_id));
}

#[test]
fn salted_hash_differs_from_unsalted() {
    let raw = "user-abc";
    let salted = hash_with_salt(raw, "s");
    let unsalted = hash_value(raw);
    assert_ne!(salted, unsalted);
}

#[test]
fn token_is_16_hex_chars() {
    let t = CorrelationToken::from_raw("x", "y");
    assert_eq!(t.as_str().len(), 16);
    assert!(t.as_str().chars().all(|c| c.is_ascii_hexdigit()));
}
