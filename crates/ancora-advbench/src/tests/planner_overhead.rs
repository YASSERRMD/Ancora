use crate::{run_all, BASELINE};

#[test]
fn planner_overhead_within_threshold() {
    let report = run_all();
    let r = report.get("planner").expect("planner result");
    assert!(
        r.elapsed_ns < BASELINE.planner_ns * 2,
        "planner elapsed {}ns exceeds 2x baseline {}ns",
        r.elapsed_ns,
        BASELINE.planner_ns
    );
}

#[test]
fn planner_quality_measured() {
    let report = run_all();
    let r = report.get("planner").expect("planner result");
    let q = r.quality.expect("quality should be set");
    assert!(q > 0.0 && q <= 1.0, "quality must be in (0, 1]");
}
