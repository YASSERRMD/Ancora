use crate::gate_e2e::{all_gates_pass, run_gates, GateVerdict, RegressionGate};

#[test]
fn regression_gate_blocks_a_bad_change() {
    let gate = RegressionGate::new("pass_rate", 0.90, 0.02);
    // Candidate that regressed below baseline.
    let result = gate.check(0.80);
    assert!(result.is_err(), "gate must block a regression");
}

#[test]
fn regression_gate_passes_a_good_change() {
    let gate = RegressionGate::new("pass_rate", 0.90, 0.02);
    let result = gate.check(0.95);
    assert!(result.is_ok(), "gate must pass an improvement");
}

#[test]
fn regression_gate_passes_within_tolerance() {
    let gate = RegressionGate::new("pass_rate", 0.90, 0.05);
    // 0.86 is within 5% tolerance below 0.90.
    let result = gate.check(0.86);
    assert!(result.is_ok(), "value within tolerance must pass");
}

#[test]
fn run_gates_returns_correct_verdicts() {
    let gates = vec![
        RegressionGate::new("accuracy", 0.85, 0.01),
        RegressionGate::new("latency_ms", 500.0, 50.0),
    ];
    let metrics = &[("accuracy", 0.88), ("latency_ms", 450.0)];
    let verdicts = run_gates(&gates, metrics);
    assert_eq!(verdicts.len(), 2);
    assert!(all_gates_pass(&verdicts));
}

#[test]
fn run_gates_blocks_on_missing_metric() {
    let gates = vec![RegressionGate::new("missing_metric", 1.0, 0.0)];
    let verdicts = run_gates(&gates, &[]);
    assert!(!all_gates_pass(&verdicts));
    assert!(matches!(&verdicts[0], GateVerdict::Block(_)));
}
