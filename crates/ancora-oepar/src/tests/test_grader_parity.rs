use crate::grader_parity::{
    check_grader_parity, reference_grader_results, ContainsGrader, ExactMatchGrader, F1Grader,
    Grader,
};

#[test]
fn test_exact_match_grader_perfect() {
    let g = ExactMatchGrader;
    assert_eq!(g.grade("hello", "hello"), 1.0);
}

#[test]
fn test_exact_match_grader_mismatch() {
    let g = ExactMatchGrader;
    assert_eq!(g.grade("hello", "world"), 0.0);
}

#[test]
fn test_exact_match_case_insensitive() {
    let g = ExactMatchGrader;
    assert_eq!(g.grade("Hello", "hello"), 1.0);
}

#[test]
fn test_contains_grader() {
    let g = ContainsGrader;
    assert_eq!(g.grade("world", "hello world"), 1.0);
    assert_eq!(g.grade("missing", "hello world"), 0.0);
}

#[test]
fn test_f1_grader_perfect() {
    let g = F1Grader;
    assert_eq!(g.grade("the cat sat", "the cat sat"), 1.0);
}

#[test]
fn test_f1_grader_partial() {
    let g = F1Grader;
    let score = g.grade("the cat sat on the mat", "the cat");
    assert!(
        score > 0.0 && score < 1.0,
        "f1 score should be partial: {}",
        score
    );
}

#[test]
fn test_grader_parity_reference_results() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let results = reference_grader_results(langs);
    let issues = check_grader_parity(&results, 1e-9);
    assert!(issues.is_empty(), "grader parity issues: {:?}", issues);
}

#[test]
fn test_grader_names_are_unique() {
    let names = &[
        ExactMatchGrader.name(),
        ContainsGrader.name(),
        F1Grader.name(),
    ];
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(unique.len(), names.len(), "grader names must be unique");
}
