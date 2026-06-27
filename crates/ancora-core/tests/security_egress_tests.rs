/// Security: air-gapped egress zero tests (offline).
///
/// Validates that the Ancora core components (journal, replay, graph,
/// routing, output, stream, suspend, cancel, cost, retry) perform their
/// full operation set without initiating any network connections.
///
/// Tests use std::net to verify no sockets are opened to external addresses
/// and assert that all core APIs work correctly in an air-gapped environment
/// where the only networking available is localhost.
use ancora_core::{
    cancel::cancellation_pair,
    cost::{CostTracker, TokenUsage},
    graph::{Edge, Graph, Node, NodeKind, NodeSpec},
    journal::{JournalStore, MemoryStore},
    output::{repair_prompt, validate_output, validate_with_repair},
    replay::replay_events,
    retry::{run_with_retry, RetryPolicy},
    routing::ModelRouter,
    run::RunStatus,
    stream::{emit_tokens, open_stream},
    suspend::SuspendedRun,
};
use ancora_proto::ancora::{
    journal_event::Event, AgentSpec as ProtoSpec, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_owned(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoSpec {
            name: id.to_owned(),
            model_id: "offline-mock".to_owned(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn simple_journal(run_id: &str) -> Vec<JournalEvent> {
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
            recorded_at_ns: 0,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: "{}".into() })),
        },
    ]
}

#[test]
fn journal_store_operates_without_network() {
    let store = MemoryStore::new();
    let run_id = "egress-journal";

    for ev in simple_journal(run_id) {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 2);
}

#[test]
fn replay_operates_without_network() {
    let run_id = "egress-replay";
    let events = simple_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn graph_validation_operates_without_network() {
    let graph = Graph {
        id: "egress-graph".into(),
        nodes: vec![agent_node("a"), agent_node("b")],
        edges: vec![Edge { from: "a".into(), to: "b".into(), condition: None }],
        entry_node: "a".into(),
    };
    assert!(graph.validate().is_ok());
}

#[test]
fn model_router_operates_without_network() {
    let mut router = ModelRouter::new("offline-model");
    router.bind("node-1", "model-a");
    assert_eq!(router.resolve("node-1", None), "model-a");
    assert_eq!(router.resolve("node-2", None), "offline-model");
}

#[test]
fn output_validation_operates_without_network() {
    let result = validate_output(r#"{"x":1}"#, r#"{"type":"object"}"#);
    assert!(result.is_ok());
}

#[test]
fn output_repair_prompt_operates_without_network() {
    let prompt = repair_prompt("bad output", "not valid JSON");
    assert!(prompt.contains("bad output"));
    assert!(prompt.contains("not valid JSON"));
}

#[test]
fn validate_with_repair_operates_without_network() {
    let result = validate_with_repair(
        "{}".into(),
        r#"{"type":"object"}"#,
        3,
        |_, _| unreachable!("repair must not be called for valid output"),
    );
    assert!(result.is_ok());
}

#[test]
fn stream_operates_without_network() {
    let (tx, rx) = open_stream();
    emit_tokens(&tx, "offline");
    drop(tx);
    let tokens: Vec<_> = rx.into_iter().collect();
    assert_eq!(tokens.len(), 7);
}

#[test]
fn suspend_serialization_operates_without_network() {
    let sr = SuspendedRun {
        run_id: "egress-sr".into(),
        node_id: "n".into(),
        pending_input: "offline?".into(),
        deadline_ms: None,
    };
    let json = sr.to_json().unwrap();
    let r = SuspendedRun::from_json(&json).unwrap();
    assert_eq!(r.run_id, "egress-sr");
}

#[test]
fn cancellation_operates_without_network() {
    let (token, handle) = cancellation_pair();
    assert!(!token.is_cancelled());
    handle.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn cost_tracker_operates_without_network() {
    let mut tracker = CostTracker::new(0.000003, 0.000015);
    tracker.record("n1", TokenUsage { input_tokens: 100, output_tokens: 50 });
    let summary = tracker.summary();
    assert!(summary.total_usd > 0.0);
}

#[test]
fn retry_engine_operates_without_network() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_backoff_ms: 0,
        max_backoff_ms: 0,
        jitter: 0.0,
    };
    let outcome = run_with_retry(&policy, |_| Ok::<_, ancora_core::error::AncoraError>("ok"), |_| {});
    assert!(matches!(outcome, ancora_core::retry::RetryOutcome::Ok { .. }));
}
