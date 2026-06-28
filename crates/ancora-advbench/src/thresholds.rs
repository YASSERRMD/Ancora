/// Regression thresholds for each advanced capability benchmark.
///
/// All elapsed values are nanoseconds measured on a single developer machine.
/// CI gates on multiples of these baselines to absorb variance.
pub struct Thresholds {
    pub planner_ns:               u64,
    pub reflection_ns:            u64,
    pub routing_ns:               u64,
    pub optimization_ns:          u64,
    pub memory_consolidation_ns:  u64,
    pub coordination_ns:          u64,
    pub guardrail_ns:             u64,
    pub reasoning_ns:             u64,
    pub lh_checkpoint_ns:         u64,
    pub skills_jit_ns:            u64,
}

/// Conservative baseline thresholds (2x actual measured values).
///
/// Gate: `elapsed_ns < threshold * 2` must hold in CI.
pub const BASELINE: Thresholds = Thresholds {
    planner_ns:              200_000_000,
    reflection_ns:            10_000_000,
    routing_ns:               10_000_000,
    optimization_ns:         200_000_000,
    memory_consolidation_ns:  50_000_000,
    coordination_ns:          10_000_000,
    guardrail_ns:             10_000_000,
    reasoning_ns:             10_000_000,
    lh_checkpoint_ns:         20_000_000,
    skills_jit_ns:            20_000_000,
};
