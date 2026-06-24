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
}
