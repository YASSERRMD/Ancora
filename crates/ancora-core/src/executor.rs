use std::sync::Arc;

use ancora_proto::ancora::{
    journal_event::Event as JournalEventVariant, HumanDecisionRequestedEvent, JournalEvent,
    NodeEnteredEvent, NodeExitedEvent,
};

use crate::error::AncoraError;
use crate::graph::{Graph, Node, NodeKind};
use crate::journal::JournalStore;
use crate::suspend::{RunOutcome, SuspendedRun};

/// Executes a single graph node given its input and returns its output.
pub trait NodeExecutor: Send + Sync {
    fn execute(&self, node: &Node, input: &str) -> Result<String, AncoraError>;
}

/// The decision returned by a verifier node.
pub enum VerifierResult {
    Approved { output: String },
    Rejected { reason: String },
}

/// Inspects a candidate output and decides whether to approve or reject it.
pub trait VerifierNode: Send + Sync {
    fn verify(&self, node: &Node, candidate: &str) -> Result<VerifierResult, AncoraError>;
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

    /// Run the graph from `entry_node`, stopping when an AwaitHuman node is reached.
    /// Returns `RunOutcome::Suspended` at that node or `RunOutcome::Completed` if the graph
    /// finishes without encountering an AwaitHuman node.
    pub fn run_until_suspend(
        &mut self,
        input: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<RunOutcome, AncoraError> {
        self.graph.validate()?;

        let mut current_id = self.graph.entry_node.clone();
        let mut current_output = input.to_string();

        loop {
            let node_kind = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == current_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                node.kind
            };

            if node_kind == NodeKind::AwaitHuman {
                self.journal_node_entered(&current_id, node_kind.to_str())?;
                self.journal_seq += 1;
                let seq = self.journal_seq;
                self.store.append(&self.run_id.clone(), JournalEvent {
                    event_id: uuid::Uuid::new_v4().to_string(),
                    run_id: self.run_id.clone(),
                    seq,
                    recorded_at_ns: 0,
                    event: Some(JournalEventVariant::HumanDecisionRequested(
                        HumanDecisionRequestedEvent {
                            prompt: current_output.clone(),
                            options: vec![],
                            timeout_at_ns: 0,
                        },
                    )),
                }).map(|_| ())?;
                return Ok(RunOutcome::Suspended(SuspendedRun {
                    run_id: self.run_id.clone(),
                    node_id: current_id,
                    pending_input: current_output,
                    deadline_ms: None,
                }));
            }

            self.journal_node_entered(&current_id, node_kind.to_str())?;

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
                None => return Ok(RunOutcome::Completed(current_output)),
            }
        }
    }

    /// Resume a run that was previously suspended at an AwaitHuman node.
    ///
    /// `decision` is treated as the output of the AwaitHuman node.
    /// Execution continues from the next node in the graph.
    pub fn resume(
        &mut self,
        suspended: &SuspendedRun,
        decision: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<RunOutcome, AncoraError> {
        self.journal_seq += 1;
        let seq = self.journal_seq;
        self.store.append(&suspended.run_id.clone(), JournalEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            run_id: suspended.run_id.clone(),
            seq,
            recorded_at_ns: 0,
            event: Some(JournalEventVariant::HumanDecisionReceived(
                ancora_proto::ancora::HumanDecisionReceivedEvent {
                    decision: decision.to_string(),
                },
            )),
        }).map(|_| ())?;

        self.journal_node_exited(&suspended.node_id, true)?;

        let mut current_output = decision.to_string();

        match self.next_node(&suspended.node_id, &current_output)? {
            None => return Ok(RunOutcome::Completed(current_output)),
            Some(next_id) => {
                let mut current_id = next_id;

                loop {
                    let node_kind = {
                        let node = self.graph.nodes.iter()
                            .find(|n| n.id == current_id)
                            .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                        node.kind
                    };

                    if node_kind == NodeKind::AwaitHuman {
                        self.journal_node_entered(&current_id, node_kind.to_str())?;
                        self.journal_seq += 1;
                        let seq2 = self.journal_seq;
                        self.store.append(&self.run_id.clone(), JournalEvent {
                            event_id: uuid::Uuid::new_v4().to_string(),
                            run_id: self.run_id.clone(),
                            seq: seq2,
                            recorded_at_ns: 0,
                            event: Some(JournalEventVariant::HumanDecisionRequested(
                                HumanDecisionRequestedEvent {
                                    prompt: current_output.clone(),
                                    options: vec![],
                                    timeout_at_ns: 0,
                                },
                            )),
                        }).map(|_| ())?;
                        return Ok(RunOutcome::Suspended(SuspendedRun {
                            run_id: self.run_id.clone(),
                            node_id: current_id,
                            pending_input: current_output,
                            deadline_ms: None,
                        }));
                    }

                    self.journal_node_entered(&current_id, node_kind.to_str())?;

                    let output = {
                        let node = self.graph.nodes.iter()
                            .find(|n| n.id == current_id)
                            .ok_or_else(|| AncoraError::NodeNotFound(current_id.clone()))?;
                        executor.execute(node, &current_output)?
                    };

                    self.journal_node_exited(&current_id, true)?;

                    current_output = output;

                    match self.next_node(&current_id, &current_output)? {
                        Some(next) => current_id = next,
                        None => return Ok(RunOutcome::Completed(current_output)),
                    }
                }
            }
        }
    }

    /// Run `worker_id` and pass the result to `verifier_id` for approval.
    ///
    /// On rejection the worker is re-executed up to `max_rework` times.
    /// Returns `Ok(output)` on approval or `Err(OutputValidation)` when the rework budget is exhausted.
    /// `max_rework == 0` means try once with no retry.
    pub fn run_with_verifier(
        &mut self,
        worker_id: &str,
        verifier_id: &str,
        input: &str,
        max_rework: u32,
        executor: &dyn NodeExecutor,
        verifier: &dyn VerifierNode,
    ) -> Result<String, AncoraError> {
        let attempts = max_rework + 1;

        for attempt in 1..=attempts {
            let candidate = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == worker_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(worker_id.to_string()))?;
                executor.execute(node, input)?
            };

            let v_node = self.graph.nodes.iter()
                .find(|n| n.id == verifier_id)
                .ok_or_else(|| AncoraError::NodeNotFound(verifier_id.to_string()))?;

            match verifier.verify(v_node, &candidate)? {
                VerifierResult::Approved { output } => return Ok(output),
                VerifierResult::Rejected { reason } => {
                    if attempt >= attempts {
                        return Err(AncoraError::OutputValidation { attempts: attempt, reason });
                    }
                }
            }
        }

        unreachable!("loop always returns")
    }

    /// Run consensus across `voter_ids`. When the top vote count is shared by multiple
    /// outputs (a tie), call `arbiter_id` with the tied outputs joined by newline to
    /// make the final decision.
    pub fn run_with_arbiter(
        &mut self,
        voter_ids: &[String],
        arbiter_id: &str,
        input: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<String, AncoraError> {
        if voter_ids.is_empty() {
            return Err(AncoraError::Internal("consensus requires at least one voter".to_string()));
        }

        let mut tallies: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for voter_id in voter_ids {
            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == *voter_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(voter_id.clone()))?;
                executor.execute(node, input)?
            };
            *tallies.entry(output).or_insert(0) += 1;
        }

        let max_votes = tallies.values().copied().max().unwrap_or(0);
        let mut tied: Vec<String> = tallies.into_iter()
            .filter(|(_, v)| *v == max_votes)
            .map(|(k, _)| k)
            .collect();
        tied.sort();

        if tied.len() == 1 {
            return Ok(tied.remove(0));
        }

        // Call arbiter to break the tie.
        let arbiter_input = tied.join("\n");
        let arbiter_node = self.graph.nodes.iter()
            .find(|n| n.id == arbiter_id)
            .ok_or_else(|| AncoraError::NodeNotFound(arbiter_id.to_string()))?;
        executor.execute(arbiter_node, &arbiter_input)
    }

    /// Run all nodes in `voter_ids` on the same `input` and return the output that
    /// received the most votes. Ties between outputs of equal vote count are broken by
    /// lexicographic order so the result is always deterministic.
    ///
    /// Returns `Err(Internal)` when `voter_ids` is empty.
    pub fn run_consensus(
        &mut self,
        voter_ids: &[String],
        input: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<String, AncoraError> {
        if voter_ids.is_empty() {
            return Err(AncoraError::Internal("consensus requires at least one voter".to_string()));
        }

        let mut tallies: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for voter_id in voter_ids {
            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == *voter_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(voter_id.clone()))?;
                executor.execute(node, input)?
            };
            *tallies.entry(output).or_insert(0) += 1;
        }

        // Pick the output with the most votes; break ties lexicographically.
        let winner = tallies.into_iter()
            .max_by(|(a_out, a_votes), (b_out, b_votes)| {
                a_votes.cmp(b_votes).then_with(|| b_out.cmp(a_out))
            })
            .map(|(output, _)| output)
            .expect("tallies is non-empty");

        Ok(winner)
    }

    /// Transfer control sequentially through `agent_ids`, passing each agent's output
    /// as the next agent's input. Returns the final agent's output.
    pub fn run_handoff(
        &mut self,
        agent_ids: &[String],
        input: &str,
        executor: &dyn NodeExecutor,
    ) -> Result<String, AncoraError> {
        if agent_ids.is_empty() {
            return Ok(input.to_string());
        }
        let mut current = input.to_string();
        for agent_id in agent_ids {
            let output = {
                let node = self.graph.nodes.iter()
                    .find(|n| n.id == *agent_id)
                    .ok_or_else(|| AncoraError::NodeNotFound(agent_id.clone()))?;
                executor.execute(node, &current)?
            };
            current = output;
        }
        Ok(current)
    }

    /// Run a round-robin group chat for up to `max_rounds` rounds.
    ///
    /// Each round cycles through all `agent_ids` in order. Each agent receives the
    /// previous agent's output as its input. Returns all (agent_id, response) pairs
    /// across all completed rounds. `max_rounds == 0` runs exactly one round.
    pub fn run_group_chat(
        &mut self,
        agent_ids: &[String],
        initial_message: &str,
        max_rounds: u32,
        executor: &dyn NodeExecutor,
    ) -> Result<Vec<(String, String)>, AncoraError> {
        let rounds = max_rounds.max(1);
        let mut results = Vec::new();
        let mut current = initial_message.to_string();

        for _round in 0..rounds {
            for agent_id in agent_ids {
                let output = {
                    let node = self.graph.nodes.iter()
                        .find(|n| n.id == *agent_id)
                        .ok_or_else(|| AncoraError::NodeNotFound(agent_id.clone()))?;
                    executor.execute(node, &current)?
                };
                current = output.clone();
                results.push((agent_id.clone(), output));
            }
        }

        Ok(results)
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

    #[test]
    fn loop_exits_on_condition_and_on_cap() {
        // Node "counter" appends "+1" to input each iteration; exits when output contains "done"
        struct CounterExecutor {
            target: u32,
        }
        impl NodeExecutor for CounterExecutor {
            fn execute(&self, _node: &Node, input: &str) -> Result<String, AncoraError> {
                let count = input.parse::<u32>().unwrap_or(0) + 1;
                if count >= self.target {
                    Ok(format!("{count}:done"))
                } else {
                    Ok(count.to_string())
                }
            }
        }

        let graph = Graph {
            id: "g-loop".to_string(),
            nodes: vec![function_node("counter")],
            edges: vec![],
            entry_node: "counter".to_string(),
        };

        // Case 1: exits on condition after 3 iterations
        let mut exec = GraphExecutor::new(graph, "run-loop-c1", Arc::new(MemoryStore::new()));
        let result = exec.run_loop_node("counter", "0", "done", 10, &CounterExecutor { target: 3 }).unwrap();
        assert!(result.contains("done"), "loop must exit when condition is met");

        // Case 2: exits on cap (condition never met because target > cap)
        let graph2 = Graph {
            id: "g-loop2".to_string(),
            nodes: vec![function_node("counter")],
            edges: vec![],
            entry_node: "counter".to_string(),
        };
        let mut exec2 = GraphExecutor::new(graph2, "run-loop-c2", Arc::new(MemoryStore::new()));
        let err = exec2.run_loop_node("counter", "0", "done", 2, &CounterExecutor { target: 99 }).unwrap_err();
        assert!(matches!(err, AncoraError::MaxSteps { max_steps: 2 }));
    }

    #[test]
    fn handoff_transfers_context_correctly() {
        let agents: Vec<String> = vec!["alice".to_string(), "bob".to_string(), "carol".to_string()];
        let graph = Graph {
            id: "g-handoff".to_string(),
            nodes: agents.iter().map(|id| function_node(id)).collect(),
            edges: vec![],
            entry_node: "alice".to_string(),
        };

        let mut exec = GraphExecutor::new(graph, "run-handoff-1", Arc::new(MemoryStore::new()));
        let result = exec.run_handoff(&agents, "start", &PrefixExecutor).unwrap();

        // PrefixExecutor produces "[id]input" so the chain is:
        // alice("start") -> "[alice]start"
        // bob("[alice]start") -> "[bob][alice]start"
        // carol("[bob][alice]start") -> "[carol][bob][alice]start"
        assert_eq!(result, "[carol][bob][alice]start");
    }

    #[test]
    fn group_chat_respects_turn_cap() {
        let agents: Vec<String> = vec!["x".to_string(), "y".to_string()];
        let graph = Graph {
            id: "g-chat".to_string(),
            nodes: agents.iter().map(|id| function_node(id)).collect(),
            edges: vec![],
            entry_node: "x".to_string(),
        };

        let mut exec = GraphExecutor::new(graph, "run-chat-1", Arc::new(MemoryStore::new()));
        let results = exec.run_group_chat(&agents, "hello", 2, &PrefixExecutor).unwrap();

        // 2 agents * 2 rounds = 4 total turns; turns alternate x, y, x, y
        assert_eq!(results.len(), 4, "turn count must equal agents * rounds");
        assert_eq!(results[0].0, "x");
        assert_eq!(results[1].0, "y");
        assert_eq!(results[2].0, "x");
        assert_eq!(results[3].0, "y");

        // With 1 round the cap produces exactly 2 turns
        let graph2 = Graph {
            id: "g-chat2".to_string(),
            nodes: agents.iter().map(|id| function_node(id)).collect(),
            edges: vec![],
            entry_node: "x".to_string(),
        };
        let mut exec2 = GraphExecutor::new(graph2, "run-chat-2", Arc::new(MemoryStore::new()));
        let one_round = exec2.run_group_chat(&agents, "hello", 1, &PrefixExecutor).unwrap();
        assert_eq!(one_round.len(), 2, "one round with 2 agents must produce exactly 2 turns");
    }

    #[test]
    fn verifier_rejection_triggers_bounded_rework() {
        use std::sync::atomic::{AtomicU32, Ordering};

        let graph = Graph {
            id: "g-verify".to_string(),
            nodes: vec![function_node("worker"), function_node("verifier")],
            edges: vec![],
            entry_node: "worker".to_string(),
        };

        // Worker appends its call count to the output.
        let call_count = Arc::new(AtomicU32::new(0));
        let cc = Arc::clone(&call_count);
        struct CountingWorker(Arc<AtomicU32>);
        impl NodeExecutor for CountingWorker {
            fn execute(&self, _node: &Node, _input: &str) -> Result<String, AncoraError> {
                let n = self.0.fetch_add(1, Ordering::SeqCst) + 1;
                Ok(format!("output-{n}"))
            }
        }

        // Verifier rejects everything.
        struct AlwaysReject;
        impl VerifierNode for AlwaysReject {
            fn verify(&self, _node: &Node, candidate: &str) -> Result<VerifierResult, AncoraError> {
                Ok(VerifierResult::Rejected { reason: format!("rejected {candidate}") })
            }
        }

        let mut exec = GraphExecutor::new(graph, "run-verify-1", Arc::new(MemoryStore::new()));
        let err = exec.run_with_verifier(
            "worker", "verifier", "in", 2, &CountingWorker(cc), &AlwaysReject,
        ).unwrap_err();

        assert!(matches!(err, AncoraError::OutputValidation { attempts: 3, .. }));
        assert_eq!(call_count.load(Ordering::SeqCst), 3, "worker must be called 1 + max_rework times");
    }

    #[test]
    fn consensus_selects_majority_result() {
        let graph = Graph {
            id: "g-consensus".to_string(),
            nodes: vec![
                function_node("v1"),
                function_node("v2"),
                function_node("v3"),
                function_node("v4"),
                function_node("v5"),
            ],
            edges: vec![],
            entry_node: "v1".to_string(),
        };

        // v1, v2, v4 return "A"; v3, v5 return "B" -> majority is "A"
        struct MajorityVoter;
        impl NodeExecutor for MajorityVoter {
            fn execute(&self, node: &Node, _input: &str) -> Result<String, AncoraError> {
                match node.id.as_str() {
                    "v3" | "v5" => Ok("B".to_string()),
                    _ => Ok("A".to_string()),
                }
            }
        }

        let voters: Vec<String> = vec!["v1", "v2", "v3", "v4", "v5"]
            .into_iter().map(|s| s.to_string()).collect();
        let mut exec = GraphExecutor::new(graph, "run-consensus-1", Arc::new(MemoryStore::new()));
        let result = exec.run_consensus(&voters, "in", &MajorityVoter).unwrap();
        assert_eq!(result, "A", "majority vote must select A (3 votes vs 2)");
    }
}
