use crate::{run_all, BASELINE};

#[test]
fn memory_consolidation_within_threshold() {
    let report = run_all();
    let r = report.get("memory_consolidation").expect("memory_consolidation result");
    assert!(
        r.elapsed_ns < BASELINE.memory_consolidation_ns * 2,
        "memory_consolidation elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn memory_consolidation_token_units_positive() {
    let report = run_all();
    let r = report.get("memory_consolidation").expect("memory_consolidation result");
    assert!(r.token_units > 0, "promoted entries should be > 0");
}
