use crate::prod_samples::{ProdEvalSet, ProdSample, Sensitivity};

#[test]
fn test_public_sample_is_eval_safe() {
    let s = ProdSample::new("id1", "gpt-4", "openai", "hello", "world", 100);
    assert!(s.is_eval_safe());
    assert_eq!(s.sensitivity, Sensitivity::Public);
}

#[test]
fn test_pii_sample_is_not_eval_safe() {
    let s = ProdSample::new("id2", "gpt-4", "openai", "secret data", "secret", 50).with_pii();
    assert!(!s.is_eval_safe());
}

#[test]
fn test_redacted_sample_becomes_eval_safe() {
    let mut s = ProdSample::new("id3", "gpt-4", "openai", "secret data", "secret", 50).with_pii();
    s.redact("[REDACTED]");
    assert!(s.is_eval_safe());
    assert_eq!(s.sensitivity, Sensitivity::Redacted);
    assert_eq!(s.prompt, "[REDACTED]");
    assert_eq!(s.response, "[REDACTED]");
}

#[test]
fn test_redact_noop_on_public() {
    let mut s = ProdSample::new("id4", "m", "p", "prompt", "response", 10);
    s.redact("[X]");
    // Should not change anything for a public sample.
    assert_eq!(s.prompt, "prompt");
    assert_eq!(s.sensitivity, Sensitivity::Public);
}

#[test]
fn test_eval_set_rejects_pii_sample() {
    let mut set = ProdEvalSet::new();
    let s = ProdSample::new("id5", "m", "p", "pii", "pii", 10).with_pii();
    let result = set.add(s);
    assert!(result.is_err());
    assert!(set.is_empty());
}

#[test]
fn test_eval_set_accepts_public_sample() {
    let mut set = ProdEvalSet::new();
    let s = ProdSample::new("id6", "claude-3", "anthropic", "q", "a", 200);
    set.add(s).unwrap();
    assert_eq!(set.len(), 1);
}

#[test]
fn test_eval_set_accepts_redacted_sample() {
    let mut set = ProdEvalSet::new();
    let mut s = ProdSample::new("id7", "m", "p", "pii", "pii", 10).with_pii();
    s.redact("[R]");
    set.add(s).unwrap();
    assert_eq!(set.len(), 1);
}

#[test]
fn test_eval_set_filter_by_model() {
    let mut set = ProdEvalSet::new();
    set.add(ProdSample::new("a", "gpt-4", "openai", "q", "a", 10))
        .unwrap();
    set.add(ProdSample::new("b", "claude-3", "anthropic", "q", "a", 10))
        .unwrap();
    set.add(ProdSample::new("c", "gpt-4", "openai", "q", "a", 10))
        .unwrap();
    let gpt4 = set.by_model("gpt-4");
    assert_eq!(gpt4.len(), 2);
}

#[test]
fn test_eval_set_filter_by_provider() {
    let mut set = ProdEvalSet::new();
    set.add(ProdSample::new("a", "gpt-4", "openai", "q", "a", 10))
        .unwrap();
    set.add(ProdSample::new("b", "claude-3", "anthropic", "q", "a", 10))
        .unwrap();
    let anthropic = set.by_provider("anthropic");
    assert_eq!(anthropic.len(), 1);
}
