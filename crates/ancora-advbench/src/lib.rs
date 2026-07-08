pub mod harness;
pub mod result;
pub mod thresholds;

pub use harness::run_all;
pub use result::{BenchReport, BenchResult};
pub use thresholds::{Thresholds, BASELINE};

#[cfg(test)]
mod tests {
    mod airgap_bench;
    mod all_caps_bench;
    mod bench_count;
    mod consolidation_reduces;
    mod coordination_overhead;
    mod guardrail_latency;
    mod harness_reproducible;
    mod harness_schema;
    mod lh_checkpoint_cost;
    mod memory_consolidation;
    mod optimization_compile;
    mod planner_overhead;
    mod reasoning_overhead;
    mod reflection_cost;
    mod reflection_quality;
    mod regression_gate;
    mod routing_cost_lower;
    mod routing_cost_savings;
    mod skills_jit_latency;
    mod summary_format;
}
