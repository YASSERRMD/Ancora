/// End-to-end MCP tool use tests (offline).
///
/// Validates the journal representation of a run where an agent invokes an
/// MCP tool, receives a result, and continues to completion. No live MCP
/// server is contacted; results come from fixture data.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, NodeEnteredEvent,
    NodeExitedEvent, RunCompletedEvent, RunStartedEvent,
};

fn ev(seq: u64, run_id: &str, event: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(event),
    }
}

fn tool_call_json(tool: &str, args: &str) -> String {
    format!(r#"{{"tool":"{}","args":{}}}"#, tool, args)
}

fn tool_result_json(tool: &str, result: &str) -> String {
    format!(r#"{{"tool":"{}","result":{}}}"#, tool, result)
}

fn build_mcp_tool_journal(run_id: &str) -> Vec<JournalEvent> {
    let call = tool_call_json("web-search", r#"{"query":"Rust async runtimes"}"#);
    let result = tool_result_json(
        "web-search",
        r#"["tokio","async-std","smol"]"#,
    );
    let final_output = r#"{"summary":"Popular Rust async runtimes are tokio, async-std, and smol."}"#;

    vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
        ev(1, run_id, Event::NodeEntered(NodeEnteredEvent {
            node_id: "agent-node".into(),
            node_kind: "agent".into(),
        })),
        ev(2, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: "tool-call-web-search-1".into(),
            activity_kind: "tool".into(),
            input_json: call,
            result_json: result,
            replayed: false,
        })),
        ev(3, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: "llm-after-tool-1".into(),
            activity_kind: "llm".into(),
            input_json: r#"{"messages":["tool result received"]}"#.into(),
            result_json: final_output.into(),
            replayed: false,
        })),
        ev(4, run_id, Event::NodeExited(NodeExitedEvent {
            node_id: "agent-node".into(),
            success: true,
        })),
        ev(5, run_id, Event::RunCompleted(RunCompletedEvent {
            output_json: final_output.into(),
        })),
    ]
}

#[test]
fn mcp_tool_use_e2e_journal_replays_to_completed() {
    let run_id = "e2e-mcp-1";
    let store = MemoryStore::new();

    for event in build_mcp_tool_journal(run_id) {
        store.append(run_id, event).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn mcp_tool_use_activity_key_order_is_tool_then_llm() {
    let run_id = "e2e-mcp-order";
    let store = MemoryStore::new();

    for event in build_mcp_tool_journal(run_id) {
        store.append(run_id, event).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[0], "tool-call-web-search-1");
    assert_eq!(state.activity_keys[1], "llm-after-tool-1");
}

#[test]
fn mcp_tool_call_json_is_valid() {
    let json = tool_call_json("search", r#"{"q":"test"}"#);
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["tool"], "search");
}

#[test]
fn mcp_tool_result_json_is_valid() {
    let json = tool_result_json("search", r#"["result1"]"#);
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["tool"], "search");
    assert!(parsed["result"].is_array());
}

#[test]
fn mcp_tool_use_exactly_two_activity_keys() {
    let run_id = "e2e-mcp-keys";
    let events = build_mcp_tool_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys.len(), 2);
}

#[test]
fn mcp_tool_use_tool_activity_kind_is_tool() {
    let events = build_mcp_tool_journal("e2e-mcp-kind");
    let tool_events: Vec<_> = events
        .iter()
        .filter(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                a.activity_kind == "tool"
            } else {
                false
            }
        })
        .collect();
    assert_eq!(tool_events.len(), 1);
}

#[test]
fn mcp_tool_use_final_output_is_valid_json() {
    let events = build_mcp_tool_journal("e2e-mcp-json");
    let output = events
        .iter()
        .find_map(|e| {
            if let Some(Event::RunCompleted(c)) = &e.event {
                Some(c.output_json.clone())
            } else {
                None
            }
        })
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(v["summary"].is_string());
}

#[test]
fn mcp_tool_use_replays_deterministically() {
    let run_id = "e2e-mcp-idem";
    let events = build_mcp_tool_journal(run_id);

    let a = replay_events(run_id, &events).unwrap();
    let b = replay_events(run_id, &events).unwrap();
    assert_eq!(a.activity_keys, b.activity_keys);
}

#[test]
fn mcp_tool_use_journal_has_six_events() {
    let events = build_mcp_tool_journal("e2e-mcp-count");
    assert_eq!(events.len(), 6);
}
