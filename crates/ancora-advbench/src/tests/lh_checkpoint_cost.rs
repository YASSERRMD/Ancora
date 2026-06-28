use crate::{run_all, BASELINE};

#[test]
fn lh_checkpoint_within_threshold() {
    let report = run_all();
    let r = report.get("lh_checkpoint").expect("lh_checkpoint result");
    assert!(
        r.elapsed_ns < BASELINE.lh_checkpoint_ns * 2,
        "lh_checkpoint elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn lh_checkpoint_count_correct() {
    let report = run_all();
    let r = report.get("lh_checkpoint").expect("lh_checkpoint result");
    // EveryN(10) over 1..=500 triggers at 10, 20, ..., 500 = 50 checkpoints
    assert_eq!(r.token_units, 50, "expected 50 checkpoints");
}
