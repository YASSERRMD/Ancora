/// End-to-end graph chain pipeline tests (offline).
///
/// Validates a linear chain graph (A -> B -> C) and a branching graph
/// (A -> B and A -> C via conditional edges), covering validation, journal
/// replay, and routing behavior along graph edges.
use ancora_core::{
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

fn linear_chain_graph(node_ids: &[&str]) -> Graph {
    let nodes: Vec<Node> = node_ids.iter().map(|id| agent_node(id)).collect();
    let edges: Vec<Edge> = node_ids
        .windows(2)
        .map(|w| Edge {
            from: w[0].to_owned(),
            to: w[1].to_owned(),
            condition: None,
        })
        .collect();
    Graph {
        id: "chain-graph".into(),
        nodes,
        edges,
        entry_node: node_ids[0].to_owned(),
    }
}

fn chain_journal(run_id: &str, node_ids: &[&str]) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-0", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];

    let mut seq = 1u64;
    for node_id in node_ids {
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::NodeEntered(NodeEnteredEvent {
                node_id: (*node_id).to_owned(),
                node_kind: "agent".into(),
            })),
        });
        seq += 1;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("{}-act", node_id),
                activity_kind: "llm".into(),
                input_json: "{}".into(),
                result_json: format!(r#"{{"from":"{}"}}"#, node_id),
                replayed: false,
            })),
        });
        seq += 1;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: (*node_id).to_owned(),
                success: true,
            })),
        });
        seq += 1;
    }

    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: "{}".into(),
        })),
    });

    events
}

#[test]
fn three_node_linear_chain_validates() {
    let graph = linear_chain_graph(&["a", "b", "c"]);
    assert!(graph.validate().is_ok());
}

#[test]
fn five_node_linear_chain_validates() {
    let graph = linear_chain_graph(&["n0", "n1", "n2", "n3", "n4"]);
    assert!(graph.validate().is_ok());
}

#[test]
fn linear_chain_has_n_minus_one_edges() {
    let nodes = ["a", "b", "c", "d"];
    let graph = linear_chain_graph(&nodes);
    assert_eq!(graph.edges.len(), nodes.len() - 1);
}

#[test]
fn chain_journal_replays_to_completed_for_three_nodes() {
    let run_id = "chain-3";
    let events = chain_journal(run_id, &["a", "b", "c"]);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn chain_journal_activity_key_order_matches_node_order() {
    let run_id = "chain-order";
    let node_ids = ["node-1", "node-2", "node-3"];
    let events = chain_journal(run_id, &node_ids);
    let state = replay_events(run_id, &events).unwrap();

    let expected_keys: Vec<String> = node_ids.iter().map(|n| format!("{}-act", n)).collect();
    assert_eq!(state.activity_keys, expected_keys);
}

#[test]
fn chain_journal_stores_in_memory_correctly() {
    let store = MemoryStore::new();
    let run_id = "chain-store";
    let events = chain_journal(run_id, &["a", "b"]);

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let loaded = store.read(run_id).unwrap();
    assert_eq!(loaded.len(), events.len());
}

#[test]
fn branching_graph_with_two_successors_validates() {
    let graph = Graph {
        id: "branch-graph".into(),
        nodes: vec![agent_node("root"), agent_node("left"), agent_node("right")],
        edges: vec![
            Edge {
                from: "root".into(),
                to: "left".into(),
                condition: Some("branch=left".into()),
            },
            Edge {
                from: "root".into(),
                to: "right".into(),
                condition: Some("branch=right".into()),
            },
        ],
        entry_node: "root".into(),
    };
    assert!(graph.validate().is_ok());
}

#[test]
fn masked_chain_journals_of_different_lengths_are_not_equal() {
    use ancora_core::journal_mask::assert_structurally_equal;
    let j3 = chain_journal("m3", &["a", "b", "c"]);
    let j4 = chain_journal("m4", &["a", "b", "c", "d"]);
    let m3 = mask_events(&j3);
    let m4 = mask_events(&j4);
    assert!(assert_structurally_equal(&m3, &m4).is_err());
}

#[test]
fn single_node_chain_is_same_as_single_agent_graph() {
    let graph = linear_chain_graph(&["solo"]);
    assert_eq!(graph.nodes.len(), 1);
    assert!(graph.edges.is_empty());
    assert!(graph.validate().is_ok());
}

#[test]
fn chain_graph_entry_node_is_first_in_chain() {
    let graph = linear_chain_graph(&["first", "second", "third"]);
    assert_eq!(graph.entry_node, "first");
}
