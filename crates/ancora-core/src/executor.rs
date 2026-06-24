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

    /// Run `node_id` repeatedly, feeding each output as the next input, until the
    /// output contains `exit_condition` or `max_iterations` is reached.
    ///
    /// Returns `AncoraError::MaxSteps` when the iteration cap fires before the condition.
    /// `max_iterations == 0` means unlimited (no cap enforced).
    pub fn run_loop_node(
        &mut self,
        node_id: &str,
        input: &str,
        exit_condition: &str,
        max_iterations: u32,
        executor: &dyn NodeExecutor,
    ) -> Result<String, AncoraError> {
        let mut current_input = input.to_string();
        let mut iteration = 0u32;

        loop {
            if max_iterations > 0 && iteration >= max_iterations {
                return Err(AncoraError::MaxSteps { max_steps: max_iterations });
            }

            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == node_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(node_id.to_string()))?;
                executor.execute(node, &current_input)?
            };

            if output.contains(exit_condition) {
                return Ok(output);
            }

            current_input = output;
            iteration += 1;
        }
    }

    /// Return node ids of all unconditional outgoing edges from `from`, sorted by node id.
    ///
    /// Sorting by node id ensures the join order is stable regardless of the order
    /// in which edges were defined or branches complete.
    pub fn fan_out_ids(&self, from: &str) -> Vec<String> {
        let mut ids: Vec<String> = self.graph.edges.iter()
            .filter(|e| e.from == from && e.condition.is_none())
            .map(|e| e.to.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Execute all parallel branches from `from` against `input`.
    ///
    /// Returns (node_id, output) pairs in sorted node-id order. Journal entries are
    /// written in the same stable order so replay produces identical journal sequences.
    pub fn run_parallel_branches(
        &mut self,
        from: &str,
        input: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<Vec<(String, String)>, AncoraError> {
        let ids = self.fan_out_ids(from);
        let mut results = Vec::with_capacity(ids.len());

        for node_id in &ids {
            let node_kind = self.graph.nodes.iter()
                .find(|n| n.id == *node_id)
                .map(|n| n.kind.to_str())
                .ok_or_else(|| AncoraError::NodeNotFound(node_id.clone()))?;

            self.journal_node_entered(node_id, node_kind)?;

            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == *node_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(node_id.clone()))?;
                executor.execute(node, input)?
            };

            self.journal_node_exited(node_id, true)?;

            results.push((node_id.clone(), output));
        }

        Ok(results)
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

    #[test]
    fn conditional_routing_picks_correct_branch() {
        // Graph: start -> left (if output contains "go-left") or right (unconditional fallback)
        let graph = Graph {
            id: "g-cond".to_string(),
            nodes: vec![function_node("start"), function_node("left"), function_node("right")],
            edges: vec![
                edge("start", "left", Some("go-left")),
                edge("start", "right", None),
            ],
            entry_node: "start".to_string(),
        };

        // Executor that returns its own id as output, ignoring input.
        struct IdExecutor;
        impl NodeExecutor for IdExecutor {
            fn execute(&self, node: &Node, _input: &str) -> Result<String, AncoraError> {
                Ok(node.id.clone())
            }
        }

        // "start" returns "start" which does not contain "go-left" -> takes unconditional edge to "right"
        let mut exec = GraphExecutor::new(graph, "run-cond-1", Arc::new(MemoryStore::new()));
        let result = exec.run("", &IdExecutor).unwrap();
        assert_eq!(result, "right");

        // A graph where the start node outputs "go-left" -> takes conditional edge to "left"
        let graph2 = Graph {
            id: "g-cond2".to_string(),
            nodes: vec![function_node("start"), function_node("left"), function_node("right")],
            edges: vec![
                edge("start", "left", Some("go-left")),
                edge("start", "right", None),
            ],
            entry_node: "start".to_string(),
        };

        struct GoLeftExecutor;
        impl NodeExecutor for GoLeftExecutor {
            fn execute(&self, node: &Node, _input: &str) -> Result<String, AncoraError> {
                if node.id == "start" {
                    Ok("go-left".to_string())
                } else {
                    Ok(node.id.clone())
                }
            }
        }

        let mut exec2 = GraphExecutor::new(graph2, "run-cond-2", Arc::new(MemoryStore::new()));
        let result2 = exec2.run("", &GoLeftExecutor).unwrap();
        assert_eq!(result2, "left");
    }

    #[test]
    fn parallel_results_join_deterministically() {
        // Fan-out from "root" to branches "c-node", "a-node", "b-node" (deliberately out of order).
        // The join must produce results sorted by node id: a-node, b-node, c-node.
        let graph = Graph {
            id: "g-par".to_string(),
            nodes: vec![
                function_node("root"),
                function_node("c-node"),
                function_node("a-node"),
                function_node("b-node"),
            ],
            edges: vec![
                edge("root", "c-node", None),
                edge("root", "a-node", None),
                edge("root", "b-node", None),
            ],
            entry_node: "root".to_string(),
        };

        let store = Arc::new(MemoryStore::new());
        let store_ref = Arc::clone(&store);
        let mut exec = GraphExecutor::new(graph, "run-par-1", store);
        let results = exec.run_parallel_branches("root", "input", &PrefixExecutor).unwrap();

        let ids: Vec<&str> = results.iter().map(|(id, _)| id.as_str()).collect();
        assert_eq!(ids, vec!["a-node", "b-node", "c-node"], "branches must join in sorted order");

        // Verify journal entries are in the same sorted order.
        let events = store_ref.read("run-par-1").unwrap();
        let node_entered_ids: Vec<String> = events.iter()
            .filter_map(|e| {
                if let Some(ancora_proto::ancora::journal_event::Event::NodeEntered(ev)) = &e.event {
                    Some(ev.node_id.clone())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(node_entered_ids, vec!["a-node", "b-node", "c-node"]);
    }
}
