use crate::{run_all, BASELINE};

#[test]
fn skills_jit_within_threshold() {
    let report = run_all();
    let r = report.get("skills_jit").expect("skills_jit result");
    assert!(
        r.elapsed_ns < BASELINE.skills_jit_ns * 2,
        "skills_jit elapsed {}ns exceeds 2x baseline",
        r.elapsed_ns
    );
}

#[test]
fn skills_jit_token_units_correct() {
    let report = run_all();
    let r = report.get("skills_jit").expect("skills_jit result");
    assert_eq!(r.token_units, 200, "loaded 200 skills");
}
