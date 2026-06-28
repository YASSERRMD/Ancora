use crate::prod_samples::{ProdEvalSet, ProdSample, Sensitivity};

/// Verify that the evaluation pipeline never processes un-redacted PII.

#[test]
fn test_pipeline_blocks_unredacted_pii() {
    let mut set = ProdEvalSet::new();
    let sample = ProdSample::new("s1", "m", "p", "my SSN is 123-45-6789", "sure", 100)
        .with_pii();
    // Attempting to add un-redacted PII must fail.
    let result = set.add(sample);
    assert!(result.is_err(), "expected error for un-redacted PII sample");
    assert!(set.is_empty());
}

#[test]
fn test_pipeline_allows_redacted_sample() {
    let mut set = ProdEvalSet::new();
    let mut sample =
        ProdSample::new("s2", "m", "p", "my SSN is 123-45-6789", "sure", 100).with_pii();
    sample.redact("[REDACTED]");
    // After redaction the sample should be accepted.
    set.add(sample).unwrap();
    assert_eq!(set.len(), 1);
}

#[test]
fn test_safe_samples_excludes_nothing_after_redaction() {
    let mut set = ProdEvalSet::new();
    let mut s1 = ProdSample::new("a", "m", "p", "pii text", "resp", 10).with_pii();
    s1.redact("[X]");
    set.add(s1).unwrap();
    set.add(ProdSample::new("b", "m", "p", "clean prompt", "resp", 10)).unwrap();
    // Both are eval-safe.
    assert_eq!(set.safe_samples().len(), 2);
}

#[test]
fn test_redaction_overwrites_both_prompt_and_response() {
    let mut s = ProdSample::new("r", "m", "p", "sensitive prompt", "sensitive response", 5)
        .with_pii();
    s.redact("[ANON]");
    assert_eq!(s.prompt, "[ANON]");
    assert_eq!(s.response, "[ANON]");
    assert_eq!(s.sensitivity, Sensitivity::Redacted);
}

#[test]
fn test_redaction_of_already_redacted_sample_is_noop() {
    // A sample that was already redacted should not be altered again.
    let mut s = ProdSample::new("r2", "m", "p", "safe prompt", "safe response", 5)
        .with_pii();
    s.redact("[A]");
    assert_eq!(s.sensitivity, Sensitivity::Redacted);
    // Calling redact again should not touch a Redacted sample (the method
    // only acts on Pii sensitivity).
    s.redact("[B]");
    assert_eq!(s.prompt, "[A]"); // still [A]
}
