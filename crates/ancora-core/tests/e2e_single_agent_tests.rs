/// End-to-end single-agent tests (fully offline, no live HTTP calls).
///
/// Each test builds a journal that represents a complete model run, then
/// exercises the replay, routing, streaming, and output-validation layers.
use ancora_core::{
    graph::{Graph, Node, NodeKind, NodeSpec},
    journal::{JournalStore, MemoryStore},
    journal_mask::mask_events,
    output::validate_output,
    replay::replay_events,
    routing::ModelRouter,
    run::RunStatus,
    stream::{emit_tokens, open_stream, StreamEvent},
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
            model_id: "mock-llm".to_owned(),
            instructions: "You are a helpful assistant.".to_owned(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 3,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn build_single_agent_journal(run_id: &str, output: &str) -> Vec<JournalEvent> {
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
                node_id: "agent-1".into(),
                node_kind: "agent".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_owned(),
            seq: 2,
            recorded_at_ns: 2_000_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "llm-step-1".into(),
                activity_kind: "llm".into(),
                input_json: r#"{"messages":[]}"#.into(),
                result_json: output.to_owned(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-3", run_id),
            run_id: run_id.to_owned(),
            seq: 3,
            recorded_at_ns: 3_000_000,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: "agent-1".into(),
                success: true,
            })),
        },
        JournalEvent {
            event_id: format!("{}-4", run_id),
            run_id: run_id.to_owned(),
            seq: 4,
            recorded_at_ns: 4_000_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: output.to_owned(),
            })),
        },
    ]
}

#[test]
fn single_agent_e2e_run_completes_successfully() {
    let run_id = "e2e-single-1";
    let output = r#"{"answer":"42"}"#;
    let store = MemoryStore::new();

    for ev in build_single_agent_journal(run_id, output) {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn single_agent_e2e_activity_key_is_recorded() {
    let run_id = "e2e-single-act";
    let store = MemoryStore::new();

    for ev in build_single_agent_journal(run_id, "{}") {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert!(state.activity_keys.contains(&"llm-step-1".to_string()));
}

#[test]
fn single_agent_e2e_graph_validates_spec() {
    let graph = Graph {
        id: "g-single".into(),
        nodes: vec![agent_node("agent-1")],
        edges: vec![],
        entry_node: "agent-1".into(),
    };
    assert!(graph.validate().is_ok(), "single-node graph must validate");
}

#[test]
fn single_agent_e2e_model_router_resolves_default() {
    let router = ModelRouter::new("claude-opus-4-8");
    let resolved = router.resolve("agent-1", None);
    assert_eq!(resolved, "claude-opus-4-8");
}

#[test]
fn single_agent_e2e_output_validates_as_json() {
    let output = r#"{"answer":"the universe"}"#;
    let schema = r#"{"type":"object"}"#;
    assert!(validate_output(output, schema).is_ok());
}

#[test]
fn single_agent_e2e_masked_journal_is_structurally_stable() {
    let events = build_single_agent_journal("mask-run", "{}");
    let masked = mask_events(&events);
    assert_eq!(masked.len(), events.len());
}

#[test]
fn single_agent_e2e_replay_is_idempotent() {
    let run_id = "e2e-idem";
    let events = build_single_agent_journal(run_id, r#"{"x":1}"#);

    let a = replay_events(run_id, &events).unwrap();
    let b = replay_events(run_id, &events).unwrap();
    assert_eq!(a.run.status, b.run.status);
    assert_eq!(a.activity_keys, b.activity_keys);
}

#[test]
fn single_agent_e2e_tokens_stream_matches_output() {
    let (tx, rx) = open_stream();
    let output = "Hello!";
    emit_tokens(&tx, output);
    drop(tx);

    let tokens: Vec<String> = rx
        .into_iter()
        .filter_map(|e| {
            if let StreamEvent::Token { text } = e {
                Some(text)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(tokens.join(""), output);
}

#[test]
fn single_agent_e2e_event_count_matches_spec() {
    let events = build_single_agent_journal("count-run", "{}");
    assert_eq!(
        events.len(),
        5,
        "started+entered+activity+exited+completed = 5"
    );
}

#[test]
fn single_agent_e2e_store_clone_shares_journal() {
    let store = MemoryStore::new();
    let run_id = "e2e-clone";

    for ev in build_single_agent_journal(run_id, "{}") {
        store.append(run_id, ev).unwrap();
    }

    let store2 = store.clone();
    let events = store2.read(run_id).unwrap();
    assert_eq!(events.len(), 5, "cloned store must share the same journal");
}

#[test]
fn single_agent_e2e_two_runs_are_isolated() {
    let store = MemoryStore::new();

    for ev in build_single_agent_journal("run-a", r#"{"v":1}"#) {
        store.append("run-a", ev).unwrap();
    }
    for ev in build_single_agent_journal("run-b", r#"{"v":2}"#) {
        store.append("run-b", ev).unwrap();
    }

    let a = store.read("run-a").unwrap();
    let b = store.read("run-b").unwrap();
    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 5);
    assert_ne!(a[0].run_id, b[0].run_id, "runs must be isolated");
}
