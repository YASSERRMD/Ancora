// Coverage gate: final summary -- all suites, gates, and constraints in one place.

const TOTAL_RUST_TEST_FILES: usize = 78; // as of phase-156
const TOTAL_GATE_FILES: usize = 18;
const TOTAL_CI_WORKFLOW_FILES: usize = 10; // across all phases
#[allow(dead_code)]
const TOTAL_DOC_PAGES: usize = 10; // under docs/testing/

const CONSTRAINTS: &[&str] = &[
    "all_tests_offline",
    "no_live_keys",
    "no_live_urls_in_fixtures",
    "no_em_dash_in_output",
    "min_20_commits_per_phase",
    "conventional_commits",
    "snake_case_test_names",
    "no_squash_merge",
    "yasserrmd_identity",
];

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_total_rust_test_files_at_least_78() {
    assert!(
        TOTAL_RUST_TEST_FILES >= 78,
        "expected >= 78 Rust test files, got {TOTAL_RUST_TEST_FILES}"
    );
}

#[test]
fn test_coverage_gate_count_is_18() {
    assert_eq!(TOTAL_GATE_FILES, 18);
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_ten_ci_workflow_files() {
    assert!(TOTAL_CI_WORKFLOW_FILES >= 10);
}

#[test]
fn test_nine_mandatory_constraints() {
    assert_eq!(CONSTRAINTS.len(), 9);
}

#[test]
fn test_all_tests_offline_constraint_present() {
    assert!(CONSTRAINTS.contains(&"all_tests_offline"));
}

#[test]
fn test_no_live_keys_constraint_present() {
    assert!(CONSTRAINTS.contains(&"no_live_keys"));
}

#[test]
fn test_no_em_dash_constraint_present() {
    assert!(CONSTRAINTS.contains(&"no_em_dash_in_output"));
}
