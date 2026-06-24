use std::sync::Arc;

use crate::error::AncoraError;
use crate::graph::{Graph, Node};
use crate::journal::JournalStore;

/// Executes a single graph node given its input and returns its output.
pub trait NodeExecutor: Send + Sync {
    fn execute(&self, node: &Node, input: &str) -> Result<String, AncoraError>;
}

/// Runs a validated `Graph` sequentially, passing each node's output as the next node's input.
pub struct GraphExecutor {
    pub graph: Graph,
    pub run_id: String,
    store: Arc<dyn JournalStore>,
}

impl GraphExecutor {
    pub fn new(graph: Graph, run_id: impl Into<String>, store: Arc<dyn JournalStore>) -> Self {
        Self { graph, run_id: run_id.into(), store }
    }

    /// Execute the graph from `entry_node` to completion, returning the final node output.
    pub fn run(&mut self, input: &str, executor: &dyn NodeExecutor) -> Result<String, AncoraError> {
        self.graph.validate()?;

        let mut current_id = self.graph.entry_node.clone();
        let mut current_output = input.to_string();

        loop {
            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == current_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                executor.execute(node, &current_output)?
            };

            current_output = output;

            match self.next_node(&current_id, &current_output)? {
                Some(next_id) => current_id = next_id,
                None => return Ok(current_output),
            }
        }
    }

    fn next_node(&self, from: &str, output: &str) -> Result<Option<String>, AncoraError> {
        let outgoing: Vec<_> = self.graph.edges.iter()
            .filter(|e| e.from == from)
            .collect();

        // Conditional edges take priority: pick the first whose condition matches.
        for edge in &outgoing {
            if let Some(cond) = &edge.condition {
                if output.contains(cond.as_str()) {
                    return Ok(Some(edge.to.clone()));
                }
            }
        }

        // Fall back to the first unconditional edge.
        for edge in &outgoing {
            if edge.condition.is_none() {
                return Ok(Some(edge.to.clone()));
            }
        }

        Ok(None)
    }
}
