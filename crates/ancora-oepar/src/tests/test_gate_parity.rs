use crate::gate_parity::{standard_gates, run_gates, check_gate_parity, RegressionGate};

#[test]
fn test_standard_gates_are_three() {
    let gates = standard_gates();
    assert_eq!(gates.len(), 3, "expected 3 standard gates");
}

#[test]
fn test_gate_passes_within_tolerance() {
    let gate = RegressionGate::new("test_gate", "metric_a", 0.85, 0.05);
    assert!(gate.passes(0.87));
    assert!(gate.passes(0.85));
    assert!(gate.passes(0.80));
}

#[test]
fn test_gate_fails_outside_tolerance() {
    let gate = RegressionGate::new("test_gate", "metric_a", 0.85, 0.05);
    assert!(!gate.passes(0.70));
    assert!(!gate.passes(0.95));
}

#[test]
fn test_run_gates_all_pass_for_good_scores() {
    let scores = &[
        ("mean_score", 0.87),
        ("p50_latency_ms", 210.0),
        ("total_cost_usd", 0.10),
    ];
    let results = run_gates("rust", scores);
    for r in &results {
        assert!(r.passed, "gate {:?} failed: {:?}", r.gate_name, r.reason);
    }
}

#[test]
fn test_gate_parity_across_languages() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let scores = &[
        ("mean_score", 0.87),
        ("p50_latency_ms", 210.0),
        ("total_cost_usd", 0.10),
    ];
    let all_results: Vec<_> = langs
        .iter()
        .flat_map(|&lang| run_gates(lang, scores))
        .collect();
    let issues = check_gate_parity(&all_results);
    assert!(issues.is_empty(), "gate parity issues: {:?}", issues);
}
