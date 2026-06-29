use crate::verifier::{
    run_verifiers, ContainsKeywordsVerifier, FnVerifier, LengthVerifier, NonEmptyVerifier,
    RequiredKeysVerifier, ValidJsonVerifier, Verdict, Verifier,
};

#[test]
fn test_non_empty_verifier_passes() {
    let v = NonEmptyVerifier;
    assert_eq!(v.verify("hello"), Verdict::Pass);
}

#[test]
fn test_non_empty_verifier_fails_on_blank() {
    let v = NonEmptyVerifier;
    assert!(v.verify("   ").is_fail());
}

#[test]
fn test_valid_json_verifier_passes_valid_json() {
    let v = ValidJsonVerifier;
    assert_eq!(v.verify(r#"{"key": 1}"#), Verdict::Pass);
}

#[test]
fn test_valid_json_verifier_catches_small_model_error() {
    let v = ValidJsonVerifier;
    let result = v.verify("Here is some prose output without JSON.");
    assert!(result.is_fail(), "should catch non-JSON output");
}

#[test]
fn test_contains_keywords_verifier() {
    let v = ContainsKeywordsVerifier {
        keywords: vec!["action".into(), "result".into()],
    };
    assert_eq!(v.verify("The action produced a result."), Verdict::Pass);
    assert!(v.verify("Nothing relevant here.").is_fail());
}

#[test]
fn test_length_verifier() {
    let v = LengthVerifier { min: 5, max: 20 };
    assert_eq!(v.verify("hello world"), Verdict::Pass);
    assert!(v.verify("hi").is_fail(), "too short");
    assert!(v.verify("a".repeat(30).as_str()).is_fail(), "too long");
}

#[test]
fn test_required_keys_verifier() {
    let v = RequiredKeysVerifier { keys: vec!["name".into(), "score".into()] };
    assert_eq!(v.verify(r#"{"name": "test", "score": 0.9}"#), Verdict::Pass);
    assert!(v.verify(r#"{"name": "only-name"}"#).is_fail());
}

#[test]
fn test_fn_verifier_custom_logic() {
    let v = FnVerifier::new("custom", |output| {
        if output.starts_with("OK:") {
            Verdict::Pass
        } else {
            Verdict::Fail("must start with OK:".into())
        }
    });
    assert_eq!(v.verify("OK: done"), Verdict::Pass);
    assert!(v.verify("not ok").is_fail());
}

#[test]
fn test_run_verifiers_all_pass() {
    let verifiers: Vec<Box<dyn Verifier>> = vec![
        Box::new(NonEmptyVerifier),
        Box::new(ValidJsonVerifier),
    ];
    let report = run_verifiers(r#"{"x": 1}"#, &verifiers);
    assert!(report.passed());
    assert_eq!(report.checks.len(), 2);
}

#[test]
fn test_run_verifiers_one_fail() {
    let verifiers: Vec<Box<dyn Verifier>> = vec![
        Box::new(NonEmptyVerifier),
        Box::new(ValidJsonVerifier),
    ];
    let report = run_verifiers("just prose", &verifiers);
    assert!(!report.passed(), "should fail when JSON verifier rejects prose");
}
