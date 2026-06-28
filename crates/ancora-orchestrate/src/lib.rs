pub mod agent_spec;
pub mod task_graph;
pub mod result_aggregator;
pub mod spawn;
pub mod error;
pub mod depth_limiter;
pub mod fanout;

#[cfg(test)]
mod tests;

pub use agent_spec::{AgentRole, AgentSpec, AgentTask};
pub use task_graph::{TaskGraph, TaskState};
pub use result_aggregator::{AgentResult, ResultAggregator};
pub use spawn::{SpawnRecord, SpawnTracker};
pub use error::OrchestrateError;
pub use depth_limiter::DepthLimiter;
pub use fanout::fan_out;
