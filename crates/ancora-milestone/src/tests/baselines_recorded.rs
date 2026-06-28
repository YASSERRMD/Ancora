use ancora_advbench::{run_all, BASELINE};

#[test]
fn advanced_baselines_recorded() {
    let report = run_all();
    assert_eq!(report.results.len(), 10, "all 10 capability baselines recorded");
}

#[test]
fn all_baselines_within_threshold() {
    let report = run_all();
    let thresholds = [
        ("planner",             BASELINE.planner_ns),
        ("reflection",          BASELINE.reflection_ns),
        ("routing",             BASELINE.routing_ns),
        ("coordination",        BASELINE.coordination_ns),
        ("guardrail",           BASELINE.guardrail_ns),
        ("reasoning",           BASELINE.reasoning_ns),
        ("lh_checkpoint",       BASELINE.lh_checkpoint_ns),
        ("skills_jit",          BASELINE.skills_jit_ns),
    ];
    for (name, threshold) in &thresholds {
        let r = report.get(name).unwrap_or_else(|| panic!("missing: {name}"));
        assert!(r.elapsed_ns < threshold * 2, "{name} regression: {}ns", r.elapsed_ns);
    }
}
