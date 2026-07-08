use crate::retrieval::RetrievalChecker;

#[test]
fn retrieval_passes_when_all_keywords_present() {
    let contents = vec![
        "the user likes rust".to_string(),
        "prefers offline mode".to_string(),
    ];
    assert!(RetrievalChecker::check(&contents, &["rust", "offline"]));
}

#[test]
fn retrieval_fails_when_keyword_missing() {
    let contents = vec!["the user likes rust".to_string()];
    assert!(!RetrievalChecker::check(&contents, &["rust", "python"]));
}

#[test]
fn missing_keywords_returns_correct_list() {
    let contents = vec!["knows rust".to_string()];
    let missing = RetrievalChecker::missing_keywords(&contents, &["rust", "go", "python"]);
    assert_eq!(missing, vec!["go", "python"]);
}

#[test]
fn empty_required_keywords_always_passes() {
    let contents = vec!["anything".to_string()];
    assert!(RetrievalChecker::check(&contents, &[]));
}
