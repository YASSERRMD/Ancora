use crate::schema::{ToolCall, ToolResult};
use crate::error::ToolError;

pub const DEFAULT_PARALLEL_LIMIT: usize = 8;

/// Groups tool calls for parallel vs sequential dispatch.
#[derive(Debug, Clone)]
pub enum DispatchGroup {
    Parallel(Vec<ToolCall>),
    Sequential(Vec<ToolCall>),
}

pub struct ParallelDispatcher {
    pub limit: usize,
}

impl ParallelDispatcher {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub fn validate_group(&self, calls: &[ToolCall]) -> Result<(), ToolError> {
        if calls.len() > self.limit {
            return Err(ToolError::ParallelLimitExceeded { limit: self.limit });
        }
        Ok(())
    }

    /// Simulate parallel execution using a provided executor closure.
    /// In production this would be async; here we call sequentially and collect results.
    pub fn execute<F>(&self, calls: Vec<ToolCall>, mut exec: F) -> Result<Vec<ToolResult>, ToolError>
    where
        F: FnMut(&ToolCall) -> Result<ToolResult, ToolError>,
    {
        self.validate_group(&calls)?;
        let mut results = vec![];
        for call in &calls {
            results.push(exec(call)?);
        }
        Ok(results)
    }
}
