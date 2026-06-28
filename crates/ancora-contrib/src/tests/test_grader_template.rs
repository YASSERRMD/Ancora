use crate::grader_template::{GradeInput, GraderError, GraderPlugin, MyGrader};

#[test]
fn grader_id_is_correct() {
    assert_eq!(MyGrader.grader_id(), "my-exact-match");
}

#[test]
fn description_is_nonempty() {
    assert!(!MyGrader.description().is_empty());
}

#[test]
fn grade_exact_match_scores_one() {
    let input = GradeInput::new("2+2", "4").with_reference("4");
    let result = MyGrader.grade(&input).expect("grade must succeed");
    assert!((result.score - 1.0).abs() < 1e-9);
    assert!(result.passed);
}

#[test]
fn grade_mismatch_scores_zero() {
    let input = GradeInput::new("2+2", "5").with_reference("4");
    let result = MyGrader.grade(&input).expect("grade must succeed");
    assert!((result.score - 0.0).abs() < 1e-9);
    assert!(!result.passed);
}

#[test]
fn grade_case_insensitive() {
    let input = GradeInput::new("capital", "HELLO").with_reference("hello");
    let result = MyGrader.grade(&input).expect("grade must succeed");
    assert!((result.score - 1.0).abs() < 1e-9);
}

#[test]
fn grade_trims_whitespace() {
    let input = GradeInput::new("q", "  answer  ").with_reference("answer");
    let result = MyGrader.grade(&input).expect("grade must succeed");
    assert!((result.score - 1.0).abs() < 1e-9);
}

#[test]
fn grade_without_reference_returns_error() {
    let input = GradeInput::new("q", "a"); // no reference
    match MyGrader.grade(&input) {
        Err(GraderError::InvalidInput(_)) => {}
        other => panic!("expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn grade_result_rationale_is_nonempty() {
    let input = GradeInput::new("q", "a").with_reference("a");
    let result = MyGrader.grade(&input).unwrap();
    assert!(!result.rationale.is_empty());
}

#[test]
fn grader_error_display() {
    assert!(!GraderError::InvalidInput("bad".into()).to_string().is_empty());
    assert!(!GraderError::GradingFailed("err".into()).to_string().is_empty());
}
