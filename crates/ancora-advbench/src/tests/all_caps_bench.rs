use crate::run_all;

const CAP_NAMES: &[&str] = &[
    "planner",
    "reflection",
    "routing",
    "optimization",
    "memory_consolidation",
    "coordination",
    "guardrail",
    "reasoning",
    "lh_checkpoint",
    "skills_jit",
];

#[test]
fn one_result_per_capability() {
    let report = run_all();
    for name in CAP_NAMES {
        let found = report.results.iter().filter(|r| r.name == *name).count();
        assert_eq!(found, 1, "expected exactly 1 result for '{name}', got {found}");
    }
}

#[test]
fn all_capabilities_covered() {
    let report = run_all();
    let names: std::collections::HashSet<&str> =
        report.results.iter().map(|r| r.name.as_str()).collect();
    for name in CAP_NAMES {
        assert!(names.contains(name), "capability '{name}' not benchmarked");
    }
}
