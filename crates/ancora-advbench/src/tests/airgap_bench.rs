/// All benchmark operations must complete in-process with no network calls.
/// This test verifies that run_all() returns successfully in an offline environment.
use crate::run_all;

#[test]
fn all_benches_complete_in_process() {
    let report = run_all();
    assert_eq!(report.results.len(), 10, "all 10 benches should complete offline");
}

#[test]
fn bench_names_non_empty() {
    let report = run_all();
    for r in &report.results {
        assert!(!r.name.is_empty(), "bench result name must not be empty");
    }
}
