use crate::parity::{all_pass, run_all, REQUIRED_FEATURES};

#[test]
fn all_languages_pass_parity() {
    assert!(
        all_pass(),
        "one or more languages failed parity: {:?}",
        run_all()
    );
}

#[test]
fn parity_reports_correct_languages() {
    let results = run_all();
    let languages: Vec<&str> = results.iter().map(|r| r.language.as_str()).collect();
    assert!(languages.contains(&"go"));
    assert!(languages.contains(&"python"));
    assert!(languages.contains(&"typescript"));
    assert!(languages.contains(&"dotnet"));
    assert!(languages.contains(&"java"));
    assert!(languages.contains(&"rust"));
}

#[test]
#[allow(clippy::const_is_empty)]
fn required_features_non_empty() {
    assert!(!REQUIRED_FEATURES.is_empty());
}

#[test]
fn no_language_has_missing_features() {
    for result in run_all() {
        assert!(
            result.missing.is_empty(),
            "{} is missing features: {:?}",
            result.language,
            result.missing
        );
    }
}
