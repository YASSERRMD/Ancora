/// Tests that traces and eval results are deterministic on replay.
/// Given the same inputs, traces and eval scores must be identical across runs.

use crate::trace_e2e::build_run_trace;
use crate::eval_e2e::{default_eval_suite, run_eval_suite};
use crate::cost_e2e::simulate_run_cost;

#[test]
fn traces_are_deterministic_on_replay() {
    let trace_a = build_run_trace("det-trace-001");
    let trace_b = build_run_trace("det-trace-001");

    assert_eq!(trace_a.trace_id, trace_b.trace_id);
    assert_eq!(trace_a.spans.len(), trace_b.spans.len());

    for (a, b) in trace_a.spans.iter().zip(trace_b.spans.iter()) {
        assert_eq!(a.span_id, b.span_id, "span ids must match on replay");
        assert_eq!(a.name, b.name, "span names must match on replay");
        assert_eq!(a.start_ns, b.start_ns, "span start times must match on replay");
        assert_eq!(a.end_ns, b.end_ns, "span end times must match on replay");
        assert_eq!(a.parent_id, b.parent_id, "span parent ids must match on replay");
        assert_eq!(a.attributes, b.attributes, "span attributes must match on replay");
    }
}

#[test]
fn eval_scores_are_deterministic_on_replay() {
    let suite_a = default_eval_suite();
    let suite_b = default_eval_suite();

    let result_a = run_eval_suite("det-eval-001", &suite_a);
    let result_b = run_eval_suite("det-eval-001", &suite_b);

    assert_eq!(result_a.scores.len(), result_b.scores.len());

    for (a, b) in result_a.scores.iter().zip(result_b.scores.iter()) {
        assert_eq!(a.case_id, b.case_id, "case ids must match on replay");
        assert_eq!(a.passed, b.passed, "pass/fail must match on replay");
        assert!((a.score - b.score).abs() < f64::EPSILON, "scores must be identical on replay");
    }
}

#[test]
fn cost_analytics_are_deterministic_on_replay() {
    let cost_a = simulate_run_cost("det-cost-001");
    let cost_b = simulate_run_cost("det-cost-001");

    assert_eq!(cost_a.total_tokens(), cost_b.total_tokens());
    assert_eq!(cost_a.total_cost_microdollars(), cost_b.total_cost_microdollars());
    assert_eq!(cost_a.entry_count(), cost_b.entry_count());
}

#[test]
fn pass_rate_is_stable_across_replays() {
    let results: Vec<f64> = (0..10)
        .map(|i| {
            let suite = default_eval_suite();
            run_eval_suite(&format!("rep-{}", i), &suite).pass_rate()
        })
        .collect();

    let first = results[0];
    for (i, &r) in results.iter().enumerate() {
        assert!(
            (r - first).abs() < f64::EPSILON,
            "pass_rate at replay {} ({}) differs from first ({})",
            i, r, first
        );
    }
}
