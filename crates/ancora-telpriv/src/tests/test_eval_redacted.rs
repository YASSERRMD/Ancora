use crate::eval_policy::{EvalPolicy, EvalSample};

#[test]
fn prompt_and_completion_dropped_by_default() {
    let policy = EvalPolicy::default();
    let sample = EvalSample {
        id: "eval-001".to_string(),
        prompt: Some("Describe the patient's symptoms.".to_string()),
        completion: Some("The patient has...".to_string()),
        score: 0.88,
        metadata: vec![],
    };
    let result = policy.apply(sample);
    assert!(result.prompt.is_none(), "prompt must be dropped");
    assert!(result.completion.is_none(), "completion must be dropped");
}

#[test]
fn score_exported_without_opt_in() {
    let policy = EvalPolicy::default();
    let sample = EvalSample {
        id: "eval-002".to_string(),
        prompt: None,
        completion: None,
        score: 0.55,
        metadata: vec![],
    };
    let result = policy.apply(sample);
    assert!((result.score - 0.55).abs() < f64::EPSILON);
}

#[test]
fn prompt_exported_when_opted_in() {
    let policy = EvalPolicy {
        allow_prompt_export: true,
        allow_completion_export: false,
        scrub_metadata: true,
    };
    let sample = EvalSample {
        id: "eval-003".to_string(),
        prompt: Some("Hello world".to_string()),
        completion: Some("secret completion".to_string()),
        score: 1.0,
        metadata: vec![],
    };
    let result = policy.apply(sample);
    assert!(result.prompt.is_some(), "prompt allowed when opted in");
    assert!(result.completion.is_none(), "completion still dropped");
}

#[test]
fn metadata_pii_scrubbed() {
    let policy = EvalPolicy::default();
    let sample = EvalSample {
        id: "eval-004".to_string(),
        prompt: None,
        completion: None,
        score: 0.7,
        metadata: vec![("annotator".to_string(), "ann@example.com".to_string())],
    };
    let result = policy.apply(sample);
    let annotator = result.metadata.iter().find(|(k, _)| k == "annotator").unwrap();
    assert!(!annotator.1.contains("ann@example.com"));
}
