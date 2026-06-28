use crate::grader::Grader;
use crate::semantic::SemanticGrader;

#[test]
fn test_identical_strings_score_one() {
    let g = SemanticGrader::new();
    let score = g.grade("the quick brown fox", "the quick brown fox");
    assert!((score.value - 1.0).abs() < 1e-9);
}

#[test]
fn test_completely_different_strings_score_low() {
    let g = SemanticGrader::new();
    let score = g.grade("apple orange banana", "car truck bicycle");
    assert!(score.value < 0.1, "score was {}", score.value);
}

#[test]
fn test_partial_overlap_scores_between_zero_and_one() {
    let g = SemanticGrader::new();
    let score = g.grade("the quick fox", "the slow fox");
    assert!(score.value > 0.0 && score.value < 1.0, "score was {}", score.value);
}

#[test]
fn test_threshold_filters_low_scores() {
    let g = SemanticGrader::new().with_threshold(0.8);
    // Partial overlap below threshold should become 0
    let score = g.grade("alpha beta", "alpha gamma delta epsilon");
    assert!((score.value - 0.0).abs() < 1e-9);
}

#[test]
fn test_grader_name() {
    let g = SemanticGrader::new();
    assert_eq!(g.name(), "semantic_jaccard");
}
