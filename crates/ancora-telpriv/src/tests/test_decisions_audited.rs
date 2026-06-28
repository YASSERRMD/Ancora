use crate::audit::{RedactionAuditLog, RedactionReason};

#[test]
fn redaction_decision_recorded() {
    let mut log = RedactionAuditLog::new();
    log.record(
        "user.email",
        RedactionReason::SensitiveField,
        "log_policy",
        true,
    );
    assert_eq!(log.len(), 1);
    let entry = &log.entries()[0];
    assert_eq!(entry.attr_name, "user.email");
    assert_eq!(entry.reason, RedactionReason::SensitiveField);
    assert!(entry.replaced);
}

#[test]
fn multiple_decisions_all_audited() {
    let mut log = RedactionAuditLog::new();
    log.record("prompt.text", RedactionReason::DropPrefix, "span_policy", false);
    log.record("api.key", RedactionReason::ClassificationThreshold, "span_policy", true);
    log.record("ip_address", RedactionReason::PiiDetected, "pii_scrub", true);
    assert_eq!(log.len(), 3);
}

#[test]
fn audit_seq_is_monotonic() {
    let mut log = RedactionAuditLog::new();
    let s0 = log.record("a", RedactionReason::NotAllowlisted, "p", true);
    let s1 = log.record("b", RedactionReason::OptInRequired, "p", false);
    let s2 = log.record("c", RedactionReason::PiiDetected, "p", true);
    assert!(s0 < s1 && s1 < s2);
}

#[test]
fn filter_by_reason_returns_correct_subset() {
    let mut log = RedactionAuditLog::new();
    log.record("f1", RedactionReason::PiiDetected, "p", true);
    log.record("f2", RedactionReason::DropPrefix, "p", false);
    log.record("f3", RedactionReason::PiiDetected, "p", true);
    log.record("f4", RedactionReason::SensitiveField, "p", true);
    let pii = log.by_reason(&RedactionReason::PiiDetected);
    assert_eq!(pii.len(), 2);
    for e in pii {
        assert_eq!(e.reason, RedactionReason::PiiDetected);
    }
}

#[test]
fn get_by_seq_works() {
    let mut log = RedactionAuditLog::new();
    let seq = log.record("x", RedactionReason::ClassificationThreshold, "eval_policy", true);
    let entry = log.get(seq).unwrap();
    assert_eq!(entry.attr_name, "x");
}
