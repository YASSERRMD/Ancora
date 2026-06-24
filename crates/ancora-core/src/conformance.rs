/// A scenario the Ancora core and all bindings must pass.
pub struct ConformanceScenario {
    pub id: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

/// Possible outcome when a binding runs a conformance scenario.
pub enum ConformanceResult {
    Passed,
    Failed { reason: String },
    Skipped { reason: String },
}

/// Single agent node runs to completion.
pub const SINGLE_AGENT: ConformanceScenario = ConformanceScenario {
    id: "single-agent",
    description: "A single agent node runs to completion without error",
    tags: &["agent", "basic"],
};

/// Agent and verifier nodes where verifier depends on agent output.
pub const MULTI_AGENT_VERIFIER: ConformanceScenario = ConformanceScenario {
    id: "multi-agent-verifier",
    description: "An agent node and a verifier node with an explicit dependency",
    tags: &["agent", "verifier", "graph"],
};

/// Run suspends for human input and resumes after approval.
pub const HUMAN_IN_LOOP: ConformanceScenario = ConformanceScenario {
    id: "human-in-loop",
    description: "A run suspends awaiting human approval and then resumes correctly",
    tags: &["suspend", "resume", "human"],
};

/// Run journal survives a simulated process restart and replays correctly.
pub const CRASH_AND_RECOVER: ConformanceScenario = ConformanceScenario {
    id: "crash-and-recover",
    description: "A run journal persists across restart and replays deterministically",
    tags: &["journal", "replay", "recovery"],
};

/// Returns all defined conformance scenarios in a stable order.
pub fn all_scenarios() -> Vec<&'static ConformanceScenario> {
    vec![
        &SINGLE_AGENT,
        &MULTI_AGENT_VERIFIER,
        &HUMAN_IN_LOOP,
        &CRASH_AND_RECOVER,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_count_is_at_least_four() {
        assert!(all_scenarios().len() >= 4);
    }

    #[test]
    fn all_scenario_ids_are_unique() {
        let scenarios = all_scenarios();
        let mut ids = std::collections::HashSet::new();
        for s in &scenarios {
            assert!(ids.insert(s.id), "duplicate scenario id: {}", s.id);
        }
    }

    #[test]
    fn all_scenario_descriptions_are_non_empty() {
        for s in all_scenarios() {
            assert!(!s.description.is_empty(), "empty description for scenario '{}'", s.id);
        }
    }

    #[test]
    fn all_scenarios_have_at_least_one_tag() {
        for s in all_scenarios() {
            assert!(!s.tags.is_empty(), "scenario '{}' has no tags", s.id);
        }
    }

    fn make_agent_node(id: &str) -> crate::graph::Node {
        use ancora_proto::ancora::AgentSpec;
        use crate::graph::{Node, NodeKind, NodeSpec};
        Node {
            id: id.to_string(),
            kind: NodeKind::Agent,
            model_id: None,
            spec: NodeSpec::Agent(AgentSpec {
                name: id.to_string(),
                model_id: "mock".to_string(),
                instructions: String::new(),
                output_schema_json: String::new(),
                tools: vec![],
                max_steps: 1,
                model_retry: None,
                model_params_json: String::new(),
            }),
        }
    }

    #[test]
    fn single_agent_scenario_graph_is_valid() {
        use crate::graph::Graph;
        let graph = Graph {
            id: "g-single".to_string(),
            nodes: vec![make_agent_node("agent1")],
            edges: vec![],
            entry_node: "agent1".to_string(),
        };
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn single_agent_scenario_graph_has_exactly_one_node() {
        use crate::graph::Graph;
        let graph = Graph {
            id: "g-single".to_string(),
            nodes: vec![make_agent_node("agent1")],
            edges: vec![],
            entry_node: "agent1".to_string(),
        };
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn multi_agent_verifier_scenario_graph_is_valid() {
        use crate::graph::{Edge, Graph};
        let graph = Graph {
            id: "g-multi".to_string(),
            nodes: vec![make_agent_node("agent"), make_agent_node("verifier")],
            edges: vec![Edge { from: "agent".into(), to: "verifier".into(), condition: None }],
            entry_node: "agent".to_string(),
        };
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn multi_agent_verifier_scenario_verifier_depends_on_agent() {
        use crate::graph::{Edge, Graph};
        let graph = Graph {
            id: "g-multi".to_string(),
            nodes: vec![make_agent_node("agent"), make_agent_node("verifier")],
            edges: vec![Edge { from: "agent".into(), to: "verifier".into(), condition: None }],
            entry_node: "agent".to_string(),
        };
        assert_eq!(graph.edges[0].from, "agent");
        assert_eq!(graph.edges[0].to, "verifier");
    }

    #[test]
    fn human_in_loop_scenario_suspended_run_round_trips_json() {
        use crate::suspend::SuspendedRun;
        let run = SuspendedRun {
            run_id: "run-1".into(),
            node_id: "approve-node".into(),
            pending_input: "approve?".into(),
            deadline_ms: None,
        };
        let json = run.to_json().unwrap();
        let recovered = SuspendedRun::from_json(&json).unwrap();
        assert_eq!(recovered.run_id, "run-1");
        assert_eq!(recovered.node_id, "approve-node");
    }

    #[test]
    fn human_in_loop_scenario_suspended_run_preserves_deadline() {
        use crate::suspend::SuspendedRun;
        let run = SuspendedRun {
            run_id: "r".into(),
            node_id: "n".into(),
            pending_input: "p".into(),
            deadline_ms: Some(9999),
        };
        let recovered = SuspendedRun::from_json(&run.to_json().unwrap()).unwrap();
        assert_eq!(recovered.deadline_ms, Some(9999));
    }

    fn make_journal_event(label: &str) -> ancora_proto::ancora::JournalEvent {
        use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};
        JournalEvent {
            event_id: label.to_string(),
            run_id: label.to_string(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: label.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".to_string(),
            })),
        }
    }

    #[test]
    fn crash_and_recover_scenario_events_survive_reuse() {
        use crate::journal::{JournalStore, MemoryStore};
        let store = MemoryStore::new();
        store.append("run-1", make_journal_event("e1")).unwrap();
        store.append("run-1", make_journal_event("e2")).unwrap();
        let events = store.read("run-1").unwrap();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn crash_and_recover_scenario_clone_shares_state() {
        use crate::journal::{JournalStore, MemoryStore};
        let store = MemoryStore::new();
        let store2 = store.clone();
        store.append("run-x", make_journal_event("e1")).unwrap();
        let events = store2.read("run-x").unwrap();
        assert_eq!(events.len(), 1, "cloned store must share the same backing state");
    }
}
