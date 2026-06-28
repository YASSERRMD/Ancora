use crate::{run_all, BASELINE};

#[test]
fn guardrail_latency_within_threshold() {
    let report = run_all();
    let r = report.get("guardrail").expect("guardrail result");
    assert!(
        r.elapsed_ns < BASELINE.guardrail_ns * 2,
        "guardrail elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn guardrail_blocks_some_inputs() {
    let report = run_all();
    let r = report.get("guardrail").expect("guardrail result");
    assert!(r.token_units > 0, "some inputs should be blocked");
}
