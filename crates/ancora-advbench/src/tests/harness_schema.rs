use crate::run_all;

const EXPECTED_NAMES: &[&str] = &[
    "planner",
    "reflection",
    "routing",
    "optimization",
    "memory_consolidation",
    "coordination",
    "guardrail",
    "reasoning",
    "lh_checkpoint",
    "skills_jit",
];

#[test]
fn bench_report_has_all_10_entries() {
    let report = run_all();
    assert_eq!(
        report.results.len(),
        EXPECTED_NAMES.len(),
        "report should have {} entries",
        EXPECTED_NAMES.len()
    );
}

#[test]
fn bench_report_names_match_schema() {
    let report = run_all();
    for name in EXPECTED_NAMES {
        assert!(
            report.get(name).is_some(),
            "missing bench result for '{name}'"
        );
    }
}

#[test]
fn bench_result_elapsed_positive() {
    let report = run_all();
    for r in &report.results {
        assert!(r.elapsed_ns > 0, "elapsed_ns must be > 0 for '{}'", r.name);
    }
}
