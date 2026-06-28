use crate::{run_all, BASELINE};

#[test]
fn all_benchmarks_within_2x_baseline() {
    let report = run_all();

    let checks = [
        ("planner",             BASELINE.planner_ns),
        ("reflection",          BASELINE.reflection_ns),
        ("routing",             BASELINE.routing_ns),
        ("optimization",        BASELINE.optimization_ns),
        ("memory_consolidation",BASELINE.memory_consolidation_ns),
        ("coordination",        BASELINE.coordination_ns),
        ("guardrail",           BASELINE.guardrail_ns),
        ("reasoning",           BASELINE.reasoning_ns),
        ("lh_checkpoint",       BASELINE.lh_checkpoint_ns),
        ("skills_jit",          BASELINE.skills_jit_ns),
    ];

    for (name, threshold) in &checks {
        let r = report.get(name).unwrap_or_else(|| panic!("missing result for {name}"));
        assert!(
            r.elapsed_ns < threshold * 2,
            "{name} elapsed {}ns exceeds 2x baseline {}ns",
            r.elapsed_ns,
            threshold
        );
    }
}
