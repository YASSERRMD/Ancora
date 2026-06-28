use crate::{run_all, BASELINE};

#[test]
fn coordination_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("coordination").expect("coordination result");
    assert!(
        r.elapsed_ns < BASELINE.coordination_ns * 2,
        "coordination elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn coordination_token_units_equals_tasks() {
    let report = run_all();
    let r = report.get("coordination").expect("coordination result");
    assert_eq!(r.token_units, 1_000, "should have 1000 journal entries");
}
