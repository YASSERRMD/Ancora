pub mod agent_spec;
pub mod depth_limiter;
pub mod error;
pub mod fanout;
pub mod result_aggregator;
pub mod spawn;
pub mod task_graph;

#[cfg(test)]
mod tests;

pub use agent_spec::{AgentRole, AgentSpec, AgentTask};
pub use depth_limiter::DepthLimiter;
pub use error::OrchestrateError;
pub use fanout::fan_out;
pub use result_aggregator::{AgentResult, ResultAggregator};
pub use spawn::{SpawnRecord, SpawnTracker};
pub use task_graph::{TaskGraph, TaskState};
