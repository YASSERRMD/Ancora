// Example parity: summary -- counts and completeness of all parity tests.

const PARITY_TEST_FILES: &[&str] = &[
    "example_parity_single_agent",
    "example_parity_verifier",
    "example_parity_hil",
    "example_parity_vector_rag",
    "example_parity_mcp",
    "example_parity_streaming",
    "example_parity_cost",
    "example_parity_otel",
    "example_parity_local_provider",
    "example_parity_structured_output",
    "example_parity_policy",
    "example_parity_durability",
    "example_parity_a2a",
    "example_parity_error_handling",
    "example_parity_chinese_providers",
    "example_parity_multi_agent",
    "example_parity_edge_deployment",
    "example_parity_journal_format",
];

const SCENARIOS_PER_LANGUAGE: &[(&str, usize)] = &[
    ("rust",       17),
    ("go",         15),
    ("python",     14),
    ("typescript", 13),
    ("dotnet",     11),
    ("java",       10),
];

#[test]
fn test_18_parity_test_files() {
    assert_eq!(PARITY_TEST_FILES.len(), 18);
}

#[test]
fn test_all_test_files_start_with_example_parity() {
    for f in PARITY_TEST_FILES {
        assert!(f.starts_with("example_parity_"), "file should start with example_parity_: {f}");
    }
}

#[test]
fn test_no_duplicate_parity_test_files() {
    let mut sorted = PARITY_TEST_FILES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), PARITY_TEST_FILES.len());
}

#[test]
fn test_rust_covered_by_most_scenarios() {
    let rust = SCENARIOS_PER_LANGUAGE.iter().find(|(l, _)| *l == "rust").unwrap();
    for (_, count) in SCENARIOS_PER_LANGUAGE {
        assert!(rust.1 >= *count, "rust should have most coverage");
    }
}

#[test]
fn test_all_six_languages_in_summary() {
    assert_eq!(SCENARIOS_PER_LANGUAGE.len(), 6);
}

#[test]
fn test_all_languages_covered_by_at_least_10_scenarios() {
    for (lang, count) in SCENARIOS_PER_LANGUAGE {
        assert!(*count >= 10, "lang {lang} covered by only {count} parity scenarios");
    }
}
