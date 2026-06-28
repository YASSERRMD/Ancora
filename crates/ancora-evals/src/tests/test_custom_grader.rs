use crate::grader::{Grader, Score};
use crate::registry::GraderRegistry;

/// A simple custom grader: scores 1.0 if the candidate contains the expected as a substring.
struct ContainsGrader;

impl Grader for ContainsGrader {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        if candidate.contains(expected) {
            Score::new(1.0).with_rationale("Substring found")
        } else {
            Score::new(0.0).with_rationale("Substring not found")
        }
    }

    fn name(&self) -> &str {
        "contains"
    }
}

#[test]
fn test_custom_grader_registered_and_runnable() {
    let mut registry = GraderRegistry::new();
    registry.register("contains", ContainsGrader);

    let score = registry.grade("contains", "hello world", "world").unwrap();
    assert!((score.value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_custom_grader_not_found_returns_error() {
    let registry = GraderRegistry::new();
    let result = registry.grade("unknown", "a", "b");
    assert!(result.is_err());
}

#[test]
fn test_registry_lists_grader_names() {
    let mut registry = GraderRegistry::new();
    registry.register("grader_a", ContainsGrader);
    registry.register("grader_b", ContainsGrader);
    let names = registry.names();
    assert!(names.contains(&"grader_a"));
    assert!(names.contains(&"grader_b"));
}

#[test]
fn test_custom_grader_scores_miss() {
    let mut registry = GraderRegistry::new();
    registry.register("contains", ContainsGrader);
    let score = registry.grade("contains", "apple", "banana").unwrap();
    assert!((score.value - 0.0).abs() < f64::EPSILON);
}
