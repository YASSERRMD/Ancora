use ancora_advbench::run_all;

#[test]
fn bench_gate_all_10_results() {
    let report = run_all();
    assert_eq!(report.results.len(), 10);
}

#[test]
fn bench_gate_routing_canonical() {
    let report = run_all();
    let r = report.get("routing").expect("routing bench");
    let q = r.quality.expect("quality set");
    assert!(
        (q - 0.8).abs() < 1e-9,
        "routing canonical quality should be 0.8, got {q}"
    );
}
