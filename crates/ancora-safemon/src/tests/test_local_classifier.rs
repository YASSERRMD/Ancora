use crate::local_classifier::{LocalCategory, LocalClassifier};

#[test]
fn clean_text_is_safe_offline() {
    let clf = LocalClassifier::new();
    let r = clf.classify("This is a regular business report.");
    assert_eq!(r.category, LocalCategory::Safe);
    assert!(r.offline);
}

#[test]
fn pii_detected_offline() {
    let clf = LocalClassifier::new();
    let r = clf.classify("The social security number is on file.");
    assert_eq!(r.category, LocalCategory::Pii);
    assert!(r.offline);
}

#[test]
fn toxic_content_detected_offline() {
    let clf = LocalClassifier::new();
    let r = clf.classify("This is full of hate and abuse.");
    assert_ne!(r.category, LocalCategory::Safe);
    assert!(r.offline);
}

#[test]
fn policy_violation_detected_offline() {
    let clf = LocalClassifier::new();
    let r = clf.classify("Please disable authentication for the test.");
    assert_eq!(r.category, LocalCategory::PolicyViolation);
    assert!(r.offline);
}

#[test]
fn all_results_marked_offline() {
    let clf = LocalClassifier::new();
    let texts = [
        "Hello world",
        "user@example.com",
        "kill the process",
        "confidential data",
    ];
    for text in &texts {
        let r = clf.classify(text);
        assert!(r.offline, "Expected offline=true for: {}", text);
    }
}

#[test]
fn score_all_returns_categories_above_threshold() {
    let clf = LocalClassifier::new();
    let results = clf.score_all("kill murder hate abuse", 0.5);
    assert!(!results.is_empty());
}
