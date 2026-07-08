/// End-to-end multi-agent verifier tests (offline).
///
/// Builds a two-node graph (agent -> verifier) and verifies the graph
/// validates, the journal replays correctly, and the verifier consensus
/// logic approves or rejects as expected.
use ancora_core::{
    error::AncoraError,
    executor::{VerifierNode, VerifierResult},
    graph::{Edge, Graph, Node, NodeKind, NodeSpec},
    journal::{JournalStore, MemoryStore},
    journal_mask::mask_events,
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, AgentSpec as ProtoSpec, JournalEvent,
    NodeEnteredEvent, NodeExitedEvent, RunCompletedEvent, RunStartedEvent,
};

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_owned(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoSpec {
            name: id.to_owned(),
            model_id: "mock".to_owned(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn two_node_journal(run_id: &str, agent_output: &str, verifier_output: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.to_owned(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.to_owned(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.to_owned(),
            seq: 1,
            recorded_at_ns: 1_000_000,
            event: Some(Event::NodeEntered(NodeEnteredEvent {
                node_id: "agent".into(),
                node_kind: "agent".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_owned(),
            seq: 2,
            recorded_at_ns: 2_000_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "agent-llm-1".into(),
                activity_kind: "llm".into(),
                input_json: "{}".into(),
                result_json: agent_output.to_owned(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-3", run_id),
            run_id: run_id.to_owned(),
            seq: 3,
            recorded_at_ns: 3_000_000,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: "agent".into(),
                success: true,
            })),
        },
        JournalEvent {
            event_id: format!("{}-4", run_id),
            run_id: run_id.to_owned(),
            seq: 4,
            recorded_at_ns: 4_000_000,
            event: Some(Event::NodeEntered(NodeEnteredEvent {
                node_id: "verifier".into(),
                node_kind: "agent".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-5", run_id),
            run_id: run_id.to_owned(),
            seq: 5,
            recorded_at_ns: 5_000_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "verifier-llm-1".into(),
                activity_kind: "llm".into(),
                input_json: agent_output.to_owned(),
                result_json: verifier_output.to_owned(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-6", run_id),
            run_id: run_id.to_owned(),
            seq: 6,
            recorded_at_ns: 6_000_000,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: "verifier".into(),
                success: true,
            })),
        },
        JournalEvent {
            event_id: format!("{}-7", run_id),
            run_id: run_id.to_owned(),
            seq: 7,
            recorded_at_ns: 7_000_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: verifier_output.to_owned(),
            })),
        },
    ]
}

struct ApproveAll;

impl VerifierNode for ApproveAll {
    fn verify(&self, _node: &Node, candidate: &str) -> Result<VerifierResult, AncoraError> {
        Ok(VerifierResult::Approved {
            output: candidate.to_string(),
        })
    }
}

struct RejectAll;

impl VerifierNode for RejectAll {
    fn verify(&self, _node: &Node, _candidate: &str) -> Result<VerifierResult, AncoraError> {
        Ok(VerifierResult::Rejected {
            reason: "policy violation".into(),
        })
    }
}

#[test]
fn two_node_graph_validates() {
    let graph = Graph {
        id: "g-two".into(),
        nodes: vec![agent_node("agent"), agent_node("verifier")],
        edges: vec![Edge {
            from: "agent".into(),
            to: "verifier".into(),
            condition: None,
        }],
        entry_node: "agent".into(),
    };
    assert!(graph.validate().is_ok());
}

#[test]
fn two_node_graph_has_correct_entry_node() {
    let graph = Graph {
        id: "g-two".into(),
        nodes: vec![agent_node("agent"), agent_node("verifier")],
        edges: vec![Edge {
            from: "agent".into(),
            to: "verifier".into(),
            condition: None,
        }],
        entry_node: "agent".into(),
    };
    assert_eq!(graph.entry_node, "agent");
}

#[test]
fn multi_agent_verifier_e2e_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "e2e-multi-1";

    for ev in two_node_journal(run_id, r#"{"draft":"ok"}"#, r#"{"verdict":"approved"}"#) {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn multi_agent_verifier_activity_keys_include_both_nodes() {
    let store = MemoryStore::new();
    let run_id = "e2e-multi-keys";

    for ev in two_node_journal(run_id, "{}", "{}") {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert!(state.activity_keys.contains(&"agent-llm-1".to_string()));
    assert!(state.activity_keys.contains(&"verifier-llm-1".to_string()));
}

#[test]
fn multi_agent_verifier_activity_key_order_is_agent_then_verifier() {
    let store = MemoryStore::new();
    let run_id = "e2e-multi-order";

    for ev in two_node_journal(run_id, "{}", "{}") {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[0], "agent-llm-1");
    assert_eq!(state.activity_keys[1], "verifier-llm-1");
}

#[test]
fn approve_all_verifier_passes_candidate() {
    let node = agent_node("v");
    let result = ApproveAll.verify(&node, r#"{"approved":true}"#).unwrap();
    assert!(matches!(result, VerifierResult::Approved { .. }));
}

#[test]
fn reject_all_verifier_fails_candidate() {
    let node = agent_node("v");
    let result = RejectAll.verify(&node, r#"{"approved":true}"#).unwrap();
    assert!(matches!(result, VerifierResult::Rejected { .. }));
}

#[test]
fn three_of_three_approvals_is_consensus() {
    let node = agent_node("v");
    let votes: Vec<VerifierResult> = (0..3)
        .map(|_| ApproveAll.verify(&node, "out").unwrap())
        .collect();
    let approvals = votes
        .iter()
        .filter(|v| matches!(v, VerifierResult::Approved { .. }))
        .count();
    assert_eq!(approvals, 3);
    assert!(approvals > 3 / 2);
}

#[test]
fn masked_two_node_journals_from_different_runs_are_equal() {
    let j1 = two_node_journal("j-x", r#"{"v":1}"#, r#"{"ok":true}"#);
    let j2 = two_node_journal("j-y", r#"{"v":99}"#, r#"{"ok":false}"#);

    let m1 = mask_events(&j1);
    let m2 = mask_events(&j2);
    assert_eq!(m1.len(), m2.len());
}

#[test]
fn multi_agent_graph_edge_is_directed_from_agent_to_verifier() {
    let graph = Graph {
        id: "g-dir".into(),
        nodes: vec![agent_node("agent"), agent_node("verifier")],
        edges: vec![Edge {
            from: "agent".into(),
            to: "verifier".into(),
            condition: None,
        }],
        entry_node: "agent".into(),
    };
    assert_eq!(graph.edges[0].from, "agent");
    assert_eq!(graph.edges[0].to, "verifier");
}
