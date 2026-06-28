use crate::{run_all, BASELINE};

#[test]
fn optimization_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("optimization").expect("optimization result");
    assert!(
        r.elapsed_ns < BASELINE.optimization_ns * 2,
        "optimization elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn optimization_quality_above_half() {
    let report = run_all();
    let r = report.get("optimization").expect("optimization result");
    let q = r.quality.expect("quality set");
    // 400 of 500 steps matched = 0.8
    assert!(q >= 0.7, "optimization quality should be >= 0.7, got {q}");
}
