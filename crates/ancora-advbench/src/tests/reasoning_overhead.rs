use crate::{run_all, BASELINE};

#[test]
fn reasoning_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("reasoning").expect("reasoning result");
    assert!(
        r.elapsed_ns < BASELINE.reasoning_ns * 2,
        "reasoning elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn reasoning_token_units_set() {
    let report = run_all();
    let r = report.get("reasoning").expect("reasoning result");
    assert!(r.token_units > 0, "citation count should be > 0");
}
