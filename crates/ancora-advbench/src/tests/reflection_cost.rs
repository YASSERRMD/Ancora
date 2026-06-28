use crate::{run_all, BASELINE};

#[test]
fn reflection_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("reflection").expect("reflection result");
    assert!(
        r.elapsed_ns < BASELINE.reflection_ns * 2,
        "reflection elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn reflection_quality_is_positive() {
    let report = run_all();
    let r = report.get("reflection").expect("reflection result");
    let q = r.quality.expect("quality set");
    assert!(q > 0.0, "reflection grew so quality must be > 0");
}
