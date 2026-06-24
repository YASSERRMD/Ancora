use std::sync::Arc;

use ancora_proto::ancora::{
    journal_event::Event as JournalEventVariant, JournalEvent, NodeEnteredEvent, NodeExitedEvent,
};

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
    journal_seq: u64,
}

impl GraphExecutor {
    pub fn new(graph: Graph, run_id: impl Into<String>, store: Arc<dyn JournalStore>) -> Self {
        Self { graph, run_id: run_id.into(), store, journal_seq: 0 }
    }

    /// Execute the graph from `entry_node` to completion, returning the final node output.
    pub fn run(&mut self, input: &str, executor: &dyn NodeExecutor) -> Result<String, AncoraError> {
        self.graph.validate()?;

        let mut current_id = self.graph.entry_node.clone();
        let mut current_output = input.to_string();

        loop {
            let node_kind = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == current_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                node.kind.to_str()
            };

            self.journal_node_entered(&current_id, node_kind)?;

            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == current_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                executor.execute(node, &current_output)?
            };

            self.journal_node_exited(&current_id, true)?;

            current_output = output;

            match self.next_node(&current_id, &current_output)? {
                Some(next_id) => current_id = next_id,
                None => return Ok(current_output),
            }
        }
    }

    fn journal_node_entered(&mut self, node_id: &str, node_kind: &str) -> Result<(), AncoraError> {
        let seq = self.journal_seq;
        self.journal_seq += 1;
        self.store.append(
            &self.run_id,
            JournalEvent {
                event_id: format!("enter:{node_id}:{seq}"),
                run_id: self.run_id.clone(),
                seq,
                recorded_at_ns: 0,
                event: Some(JournalEventVariant::NodeEntered(NodeEnteredEvent {
                    node_id: node_id.to_string(),
                    node_kind: node_kind.to_string(),
                })),
            },
        ).map(|_| ())
    }

    fn journal_node_exited(&mut self, node_id: &str, success: bool) -> Result<(), AncoraError> {
        let seq = self.journal_seq;
        self.journal_seq += 1;
        self.store.append(
            &self.run_id,
            JournalEvent {
                event_id: format!("exit:{node_id}:{seq}"),
                run_id: self.run_id.clone(),
                seq,
                recorded_at_ns: 0,
                event: Some(JournalEventVariant::NodeExited(NodeExitedEvent {
                    node_id: node_id.to_string(),
                    success,
                })),
            },
        ).map(|_| ())
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::graph::{Edge, NodeKind, NodeSpec};
    use crate::journal::MemoryStore;

    fn function_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Function,
            spec: NodeSpec::Function { name: id.to_string() },
        }
    }

    fn edge(from: &str, to: &str, condition: Option<&str>) -> Edge {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
            condition: condition.map(|s| s.to_string()),
        }
    }

    struct PrefixExecutor;
    impl NodeExecutor for PrefixExecutor {
        fn execute(&self, node: &Node, input: &str) -> Result<String, AncoraError> {
            Ok(format!("[{}]{}", node.id, input))
        }
    }

    #[test]
    fn sequential_graph_runs_in_order() {
        let graph = Graph {
            id: "g-seq".to_string(),
            nodes: vec![function_node("a"), function_node("b"), function_node("c")],
            edges: vec![edge("a", "b", None), edge("b", "c", None)],
            entry_node: "a".to_string(),
        };
        let mut exec = GraphExecutor::new(graph, "run-seq-1", Arc::new(MemoryStore::new()));
        let result = exec.run("start", &PrefixExecutor).unwrap();
        assert_eq!(result, "[c][b][a]start");
    }
}
