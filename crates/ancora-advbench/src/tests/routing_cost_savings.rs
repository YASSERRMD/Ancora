use crate::{run_all, BASELINE};

#[test]
fn routing_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("routing").expect("routing result");
    assert!(
        r.elapsed_ns < BASELINE.routing_ns * 2,
        "routing elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn routing_token_units_set() {
    let report = run_all();
    let r = report.get("routing").expect("routing result");
    assert!(r.token_units > 0, "routing token_units should be > 0");
}
