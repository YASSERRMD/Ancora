use crate::grader::Grader;
use crate::llm_judge::{Criterion, LlmJudgeGrader, Rubric};

fn sample_rubric() -> Rubric {
    Rubric::new()
        .add_criterion(Criterion::new(
            "accuracy",
            "Is the answer factually correct?",
            0.6,
        ))
        .add_criterion(Criterion::new(
            "clarity",
            "Is the answer clearly stated?",
            0.4,
        ))
}

#[test]
fn test_llm_judge_offline_scores_perfect_match() {
    let rubric = sample_rubric();
    let g = LlmJudgeGrader::offline(rubric);
    // When candidate contains all expected words, score should be high
    let score = g.grade("Paris is the capital of France", "Paris capital France");
    assert!(
        score.value > 0.8,
        "Expected high score, got {}",
        score.value
    );
}

#[test]
fn test_llm_judge_offline_scores_zero_for_mismatch() {
    let rubric = sample_rubric();
    let g = LlmJudgeGrader::offline(rubric);
    // When candidate shares no words with expected
    let score = g.grade("xyz zyx", "apple banana cherry");
    assert!(score.value < 0.2, "Expected low score, got {}", score.value);
}

#[test]
fn test_llm_judge_empty_rubric() {
    let rubric = Rubric::new();
    let g = LlmJudgeGrader::offline(rubric);
    let score = g.grade("anything", "anything");
    assert!(
        (score.value - 0.0).abs() < f64::EPSILON,
        "Empty rubric should score 0"
    );
}

#[test]
fn test_llm_judge_custom_fn() {
    let rubric =
        Rubric::new().add_criterion(Criterion::new("length", "Is the answer long enough?", 1.0));
    let g = LlmJudgeGrader::new(
        rubric,
        Box::new(|cand, _exp, _desc| if cand.len() > 5 { 1.0 } else { 0.0 }),
    );
    assert!((g.grade("short", "x").value - 0.0).abs() < f64::EPSILON);
    assert!((g.grade("long enough string", "x").value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_grader_name() {
    let g = LlmJudgeGrader::offline(Rubric::new());
    assert_eq!(g.name(), "llm_judge");
}
