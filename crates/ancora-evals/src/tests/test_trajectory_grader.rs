use crate::grader::Grader;
use crate::trajectory::TrajectoryGrader;

#[test]
fn test_trajectory_perfect_match() {
    let g = TrajectoryGrader::new();
    let expected = "search\nread_file\nwrite_file";
    let candidate = "search\nread_file\nwrite_file";
    let score = g.grade(candidate, expected);
    assert!((score.value - 1.0).abs() < f64::EPSILON, "score: {}", score.value);
}

#[test]
fn test_trajectory_subsequence_match() {
    let g = TrajectoryGrader::new();
    let expected = "search\nwrite_file";
    // Candidate has extra steps but expected tools appear in order
    let candidate = "search\nread_file\nwrite_file";
    let score = g.grade(candidate, expected);
    assert!((score.value - 1.0).abs() < f64::EPSILON, "subsequence should score 1.0, got {}", score.value);
}

#[test]
fn test_trajectory_partial_match() {
    let g = TrajectoryGrader::new();
    let expected = "search\nread_file\nwrite_file";
    let candidate = "search";
    let score = g.grade(candidate, expected);
    assert!(score.value > 0.0 && score.value < 1.0, "score: {}", score.value);
}

#[test]
fn test_trajectory_strict_mode() {
    let g = TrajectoryGrader::new().strict();
    let expected = "search\nwrite_file";
    // Extra step in between fails strict matching for position 1
    let candidate = "search\nread_file\nwrite_file";
    let score = g.grade(candidate, expected);
    assert!(score.value < 1.0, "strict mode should penalize extra steps");
}

#[test]
fn test_trajectory_empty_expected() {
    let g = TrajectoryGrader::new();
    let score = g.grade("anything", "");
    assert!((score.value - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_grader_name() {
    let g = TrajectoryGrader::new();
    assert_eq!(g.name(), "trajectory");
}
