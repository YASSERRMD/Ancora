//! Integration tests for tool-usage drift detection.

use crate::reference::ReferenceBuilder;
use crate::tool_drift::ToolDriftDetector;
use std::collections::HashMap;

fn baseline() -> crate::reference::ReferenceDistribution {
    let mut b = ReferenceBuilder::new();
    for _ in 0..100 {
        b.add("q", "a", 100, 50, &["search".to_string()], "openai");
    }
    b.build().unwrap()
}

#[test]
fn tool_drift_detected_when_tool_disappears() {
    let reference = baseline();
    // No tools used at all in current window.
    let counts: HashMap<String, usize> = HashMap::new();
    let detector = ToolDriftDetector::new(0.15);
    let result = detector.check(&reference, &counts);
    assert!(result.any_drifted, "expected tool drift when tool disappears");
}

#[test]
fn tool_drift_detected_on_new_dominant_tool() {
    let reference = baseline();
    // "code_exec" now dominates, "search" is gone.
    let mut counts = HashMap::new();
    counts.insert("code_exec".to_string(), 100);
    let detector = ToolDriftDetector::new(0.15);
    let result = detector.check(&reference, &counts);
    assert!(result.any_drifted);
}

#[test]
fn tool_drift_not_detected_on_same_distribution() {
    let reference = baseline();
    // Exactly the same tool distribution as reference.
    let mut counts = HashMap::new();
    counts.insert("search".to_string(), 50);
    let detector = ToolDriftDetector::new(0.15);
    let result = detector.check(&reference, &counts);
    assert!(!result.any_drifted);
}
