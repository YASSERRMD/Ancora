pub mod harness;
pub mod result;
pub mod thresholds;

pub use harness::run_all;
pub use result::{BenchReport, BenchResult};
pub use thresholds::{Thresholds, BASELINE};

#[cfg(test)]
mod tests {
    mod planner_overhead;
    mod reflection_cost;
    mod routing_cost_savings;
    mod optimization_compile;
    mod memory_consolidation;
    mod coordination_overhead;
    mod guardrail_latency;
    mod reasoning_overhead;
    mod lh_checkpoint_cost;
    mod skills_jit_latency;
    mod routing_cost_lower;
    mod consolidation_reduces;
    mod reflection_quality;
    mod harness_schema;
    mod harness_reproducible;
    mod regression_gate;
    mod summary_format;
    mod airgap_bench;
    mod all_caps_bench;
    mod bench_count;
}
