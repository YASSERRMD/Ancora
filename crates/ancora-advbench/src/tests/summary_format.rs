use crate::run_all;

#[test]
fn summary_contains_header_line() {
    let report = run_all();
    let s = report.summary();
    assert!(s.contains("name"), "summary should have 'name' header");
    assert!(s.contains("elapsed_ns"), "summary should have 'elapsed_ns' header");
}

#[test]
fn summary_contains_all_bench_names() {
    let report = run_all();
    let s = report.summary();
    for r in &report.results {
        assert!(s.contains(&r.name), "summary should contain bench '{}'", r.name);
    }
}

#[test]
fn summary_is_multiline() {
    let report = run_all();
    let s = report.summary();
    let lines = s.lines().count();
    assert!(lines >= 11, "summary should have at least 11 lines (header + 10 benches), got {lines}");
}
