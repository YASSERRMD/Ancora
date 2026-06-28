use crate::pii_scrub::scrub_pii;
use crate::hashing::hash_value;
use crate::span_policy::{SpanPolicy, apply_span_policy};
use crate::classification::DataClass;

#[test]
fn pii_scrub_reproducible() {
    let input = "contact alice@example.com or call 192.168.1.1";
    let first = scrub_pii(input);
    let second = scrub_pii(input);
    assert_eq!(first, second, "scrub output must be deterministic");
}

#[test]
fn hash_reproducible_on_replay() {
    let value = "session-replay-id";
    let h1 = hash_value(value);
    let h2 = hash_value(value);
    assert_eq!(h1, h2);
}

#[test]
fn span_policy_deterministic() {
    let policy = SpanPolicy::default();
    let attrs = vec![
        ("user.email", "eve@corp.io", DataClass::Sensitive),
        ("span.name", "process", DataClass::Public),
        ("prompt.text", "raw text", DataClass::Public),
    ];
    let out1 = apply_span_policy(&policy, &attrs);
    let out2 = apply_span_policy(&policy, &attrs);
    assert_eq!(out1, out2, "policy application must be deterministic");
}

#[test]
fn redacted_value_is_fixed_token() {
    // Replay with different original values for same key should still produce
    // the same redaction marker (not the original value).
    let policy = SpanPolicy::default();
    let attrs1 = vec![("user.email", "a@b.com", DataClass::Sensitive)];
    let attrs2 = vec![("user.email", "c@d.com", DataClass::Sensitive)];
    let out1 = apply_span_policy(&policy, &attrs1);
    let out2 = apply_span_policy(&policy, &attrs2);
    assert_eq!(out1[0].1, "[REDACTED]");
    assert_eq!(out2[0].1, "[REDACTED]");
}
