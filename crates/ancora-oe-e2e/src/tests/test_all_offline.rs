use crate::cost_e2e::simulate_run_cost;
use crate::eval_e2e::{default_eval_suite, run_eval_suite};
use crate::gate_e2e::{all_gates_pass, run_gates, RegressionGate};
use crate::privacy_e2e::default_redactor;
use crate::safety_e2e::default_safety_monitor;
/// Tests that verify the entire suite operates without network calls.
/// All components use in-memory mocks only.
use crate::trace_e2e::{build_run_trace, MockCollector, TraceExporter};

#[test]
fn all_components_work_offline_with_local_judge() {
    // 1. Build a trace offline.
    let trace = build_run_trace("offline-run-001");
    assert!(trace.is_complete());

    // 2. Export to in-memory collector (no network).
    let mut collector = MockCollector::new();
    collector
        .export(&trace)
        .expect("offline export must succeed");
    assert_eq!(collector.count(), 1);

    // 3. Cost analytics offline.
    let cost = simulate_run_cost("offline-run-001");
    assert!(!cost.is_empty());

    // 4. Eval offline with local exact-match judge.
    let suite = default_eval_suite();
    let eval_result = run_eval_suite("offline-suite", &suite);
    assert_eq!(eval_result.pass_rate(), 1.0);

    // 5. Regression gate offline.
    let gates = vec![RegressionGate::new("pass_rate", 0.80, 0.05)];
    let verdicts = run_gates(&gates, &[("pass_rate", eval_result.pass_rate())]);
    assert!(all_gates_pass(&verdicts));

    // 6. Safety monitor offline.
    let monitor = default_safety_monitor();
    assert!(monitor.is_safe("The capital of France is Paris."));

    // 7. Redaction offline.
    let redactor = default_redactor();
    let out = redactor.redact("No PII in this text.");
    assert!(!redactor.has_sensitive_data(&out));
}

#[test]
fn local_judge_scores_without_network() {
    let suite = default_eval_suite();
    let result = run_eval_suite("local-judge-suite", &suite);
    // All correct answers passed without any API call.
    assert!(result.all_passed());
}
