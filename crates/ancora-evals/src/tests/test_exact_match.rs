use crate::exact_match::ExactMatchGrader;
use crate::grader::Grader;

#[test]
fn test_exact_match_returns_one_for_identical_strings() {
    let g = ExactMatchGrader::new();
    let score = g.grade("hello world", "hello world");
    assert!((score.value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_exact_match_returns_zero_for_different_strings() {
    let g = ExactMatchGrader::new();
    let score = g.grade("foo", "bar");
    assert!((score.value - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_case_insensitive_mode() {
    let g = ExactMatchGrader::new().case_insensitive();
    assert!((g.grade("Hello", "hello").value - 1.0).abs() < f64::EPSILON);
    assert!((g.grade("WORLD", "world").value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_trim_mode() {
    let g = ExactMatchGrader::new().trimmed();
    assert!((g.grade("  hello  ", "hello").value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_grader_name() {
    let g = ExactMatchGrader::new();
    assert_eq!(g.name(), "exact_match");
}
