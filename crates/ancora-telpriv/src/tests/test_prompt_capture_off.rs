use crate::eval_policy::{EvalPolicy, EvalSample};
use crate::opt_in::{OptInFeature, OptInRegistry};

#[test]
fn prompt_capture_disabled_by_default() {
    let registry = OptInRegistry::new();
    assert!(
        !registry.is_enabled(&OptInFeature::PromptCapture),
        "prompt capture must be off by default"
    );
}

#[test]
fn eval_prompt_not_exported_without_opt_in() {
    // EvalPolicy::default has allow_prompt_export = false.
    let policy = EvalPolicy::default();
    let sample = EvalSample {
        id: "e100".to_string(),
        prompt: Some("confidential question".to_string()),
        completion: Some("confidential answer".to_string()),
        score: 0.5,
        metadata: vec![],
    };
    let result = policy.apply(sample);
    assert!(result.prompt.is_none());
    assert!(result.completion.is_none());
}

#[test]
fn all_features_off_by_default() {
    let registry = OptInRegistry::new();
    let features = [
        OptInFeature::PromptCapture,
        OptInFeature::CompletionCapture,
        OptInFeature::UserIdCorrelation,
        OptInFeature::EvalTextExport,
        OptInFeature::FullStackTraces,
    ];
    for feature in &features {
        assert!(
            !registry.is_enabled(feature),
            "{:?} should be off by default",
            feature
        );
    }
}

#[test]
fn explicit_opt_in_enables_prompt_capture() {
    let mut registry = OptInRegistry::new();
    registry.enable(OptInFeature::PromptCapture);
    assert!(registry.is_enabled(&OptInFeature::PromptCapture));
    // Other features remain off.
    assert!(!registry.is_enabled(&OptInFeature::CompletionCapture));
}
