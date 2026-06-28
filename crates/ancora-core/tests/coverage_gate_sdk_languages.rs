// Coverage gate: all 6 supported language SDKs have test coverage.

const SDK_LANGUAGES: &[&str] = &["rust", "go", "python", "typescript", "dotnet", "java"];

const SDK_COVERAGE: &[(&str, &str)] = &[
    ("rust",       "xlang_single_agent_rust"),
    ("go",         "xlang_single_agent_go"),
    ("python",     "xlang_single_agent_python"),
    ("typescript", "xlang_single_agent_typescript"),
    ("dotnet",     "xlang_single_agent_dotnet"),
    ("java",       "xlang_single_agent_java"),
];

const SDK_VERIFIER_COVERAGE: &[(&str, &str)] = &[
    ("rust",       "xlang_verifier_rust"),
    ("go",         "xlang_verifier_go"),
    ("python",     "xlang_verifier_python"),
    ("typescript", "xlang_verifier_typescript"),
    ("dotnet",     "xlang_verifier_dotnet"),
    ("java",       "xlang_verifier_java"),
];

const SDK_HIL_COVERAGE: &[(&str, &str)] = &[
    ("rust",       "xlang_humaninloop_rust"),
    ("go",         "xlang_humaninloop_go"),
    ("python",     "xlang_humaninloop_python"),
    ("typescript", "xlang_humaninloop_typescript"),
    ("dotnet",     "xlang_humaninloop_dotnet"),
    ("java",       "xlang_humaninloop_java"),
];

#[test]
fn test_all_languages_have_single_agent_coverage() {
    let langs: Vec<&str> = SDK_COVERAGE.iter().map(|(l, _)| *l).collect();
    for lang in SDK_LANGUAGES { assert!(langs.contains(lang), "no single-agent coverage for {lang}"); }
}

#[test]
fn test_all_languages_have_verifier_coverage() {
    let langs: Vec<&str> = SDK_VERIFIER_COVERAGE.iter().map(|(l, _)| *l).collect();
    for lang in SDK_LANGUAGES { assert!(langs.contains(lang), "no verifier coverage for {lang}"); }
}

#[test]
fn test_all_languages_have_hil_coverage() {
    let langs: Vec<&str> = SDK_HIL_COVERAGE.iter().map(|(l, _)| *l).collect();
    for lang in SDK_LANGUAGES { assert!(langs.contains(lang), "no hil coverage for {lang}"); }
}

#[test]
fn test_six_languages_defined() {
    assert_eq!(SDK_LANGUAGES.len(), 6);
}

#[test]
fn test_coverage_tables_all_have_six_entries() {
    assert_eq!(SDK_COVERAGE.len(), 6);
    assert_eq!(SDK_VERIFIER_COVERAGE.len(), 6);
    assert_eq!(SDK_HIL_COVERAGE.len(), 6);
}

#[test]
fn test_no_duplicate_language_in_coverage() {
    let mut langs: Vec<&str> = SDK_COVERAGE.iter().map(|(l, _)| *l).collect();
    langs.sort();
    langs.dedup();
    assert_eq!(langs.len(), 6);
}
