//! ancora-edgeeval: edge and small-model evaluation harness for Ancora.
//!
//! Provides evals tuned for edge constraints and small language models:
//! - Small-model capability eval suite
//! - Latency-on-device metric
//! - Memory-footprint metric
//! - Power-proxy metric
//! - Quantization-quality tradeoff eval
//! - SLM reliability eval
//! - Offline-only eval mode
//! - Edge eval report
//! - Model recommendation by device

pub mod memory;
pub mod model;
pub mod offline;
pub mod power;
pub mod quant;
pub mod recommend;
pub mod reliability;
pub mod report;
pub mod runtime;

pub use memory::{smallest_fitting, MemoryBudget};
pub use model::{CapabilitySample, SampleResult, SmallModel, SmallModelSuite, TaskCategory};
pub use offline::{MockScorer, OfflineConfig, OfflineDataset, OfflineEvalRunner, OfflineSample};
pub use power::{most_efficient, ThermalEnvelope};
pub use quant::{QuantFormat, QuantMeasurement, QuantTradeoffEval};
pub use recommend::{DeviceProfile, DeviceRecommender, ModelCandidate, Recommendation};
pub use reliability::{
    CalibrationEval, ConsistencyChecker, ReliabilityResult, ReliabilityScenario, SlmReliabilityEval,
};
pub use report::{EdgeEvalReport, ModelEvalSummary};
pub use runtime::{LatencyEvaluator, LatencyMeasurement, MemoryFootprint, PowerProxy};

#[cfg(test)]
mod tests;
