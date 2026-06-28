use crate::run_all;

#[test]
fn bench_count_is_exactly_10() {
    let report = run_all();
    assert_eq!(
        report.results.len(),
        10,
        "bench suite must cover exactly 10 capability areas"
    );
}

#[test]
fn bench_result_name_unique() {
    use std::collections::HashSet;
    let report = run_all();
    let mut seen: HashSet<&str> = HashSet::new();
    for r in &report.results {
        assert!(
            seen.insert(r.name.as_str()),
            "duplicate bench name '{}'",
            r.name
        );
    }
}
