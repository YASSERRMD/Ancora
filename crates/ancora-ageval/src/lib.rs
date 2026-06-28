//! ancora-ageval: behavior-level evaluation harness for Ancora agent capabilities.
//!
//! Provides quantitative metrics for: planning quality, reflection improvement,
//! routing cost-quality trade-offs, coordination success, guardrail catch rate,
//! reasoning correctness, and memory retention.
//!
//! Also provides regression baseline storage and eval report generation.

pub mod baseline;
pub mod coordination;
pub mod dataset;
pub mod guardrail_metric;
pub mod memory_metric;
pub mod metric;
pub mod planning;
pub mod reasoning_metric;
pub mod reflection;
pub mod report;
pub mod routing;

pub use baseline::{BaselineResult, BaselineStore};
pub use coordination::CoordinationMetric;
pub use dataset::{EvalDataset, EvalSample};
pub use guardrail_metric::GuardrailMetric;
pub use memory_metric::MemoryMetric;
pub use metric::MetricScore;
pub use planning::PlanningMetric;
pub use reasoning_metric::ReasoningMetric;
pub use reflection::ReflectionMetric;
pub use report::EvalReport;
pub use routing::RoutingMetric;

#[cfg(test)]
mod tests;
