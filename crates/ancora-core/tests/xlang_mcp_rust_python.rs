/// Cross-language MCP: Rust server consumed by Python client (offline fixture).
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_mcp_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "mcp-rust-server/search".into(),
                activity_kind: "tool-call".into(),
                input_json: r#"{"query":"xlang search","client_lang":"python","server_lang":"rust"}"#.into(),
                result_json: r#"[{"id":"r1","text":"Rust MCP result","server":"rust-mcp-server"}]"#.into(),
                replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"mcp_ok":true}"#.into() })) },
    ]
}

#[test] fn mcp_rust_server_journal_completes() {
    use ancora_core::{journal::{JournalStore, MemoryStore}, replay::replay_events, run::RunStatus};
    let store = MemoryStore::new();
    let rid = "mcp-rust-python";
    for ev in &build_mcp_journal(rid) { store.append(rid, ev.clone()).unwrap(); }
    assert_eq!(replay_events(rid, &store.read(rid).unwrap()).unwrap().run.status, RunStatus::Completed);
}

#[test] fn mcp_activity_key_contains_server_lang() {
    let evs = build_mcp_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &evs[1].event {
        assert!(a.activity_key.contains("rust"));
        assert!(a.input_json.contains("client_lang"));
        assert!(a.input_json.contains("python"));
    }
}

#[test] fn mcp_result_has_server_field() {
    let evs = build_mcp_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &evs[1].event {
        assert!(a.result_json.contains("rust-mcp-server"));
    }
}

#[test] fn mcp_activity_kind_is_tool_call() {
    let evs = build_mcp_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &evs[1].event {
        assert_eq!(a.activity_kind, "tool-call");
    }
}
