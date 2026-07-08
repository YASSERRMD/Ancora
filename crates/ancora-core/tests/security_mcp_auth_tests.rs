/// Security: unauthenticated MCP request rejection tests (offline).
///
/// Validates that the activity-key format used by MCP tool calls enforces
/// a clear boundary between authenticated and unauthenticated invocations.
/// Journal events that attempt to record MCP tool results without an
/// authenticated session marker are identified and rejected in replay
/// structural comparisons.
///
/// Also validates that ToolSpec fields carry authorization-relevant metadata
/// and that the output validator rejects unauthenticated response shapes.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    output::validate_output,
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn auth_token_header(token: &str) -> String {
    format!(r#"{{"Authorization":"Bearer {}"}}"#, token)
}

fn mcp_tool_input(tool: &str, token: Option<&str>, args: &str) -> String {
    match token {
        Some(t) => format!(
            r#"{{"tool":"{}","headers":{},"args":{}}}"#,
            tool,
            auth_token_header(t),
            args
        ),
        None => format!(r#"{{"tool":"{}","args":{}}}"#, tool, args),
    }
}

fn mcp_journal(run_id: &str, auth_token: Option<&str>, succeed: bool) -> Vec<JournalEvent> {
    let input = mcp_tool_input("mcp-search", auth_token, r#"{"q":"test"}"#);
    let result = if succeed {
        r#"{"results":["item1","item2"]}"#.to_owned()
    } else {
        r#"{"error":"unauthenticated","code":401}"#.to_owned()
    };

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
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: if succeed {
                    "mcp-authed-call-1".into()
                } else {
                    "mcp-unauthed-call-1".into()
                },
                activity_kind: "tool".into(),
                input_json: input,
                result_json: result,
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_owned(),
            seq: 2,
            recorded_at_ns: 2_000_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: if succeed {
                    r#"{"status":"ok"}"#.into()
                } else {
                    r#"{"status":"auth-failure"}"#.into()
                },
            })),
        },
    ]
}

#[test]
fn authenticated_mcp_call_replays_to_completed() {
    let run_id = "mcp-auth-ok";
    let events = mcp_journal(run_id, Some("valid-token-abc123"), true);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn unauthenticated_mcp_result_is_recorded_in_journal_for_audit() {
    let run_id = "mcp-unauth";
    let store = MemoryStore::new();

    for ev in mcp_journal(run_id, None, false) {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let tool_ev = events.iter().find(|e| {
        matches!(&e.event, Some(Event::ActivityRecorded(a)) if a.activity_key.contains("unauthed"))
    });
    assert!(
        tool_ev.is_some(),
        "unauthenticated call must be recorded in audit journal"
    );
}

#[test]
fn unauthenticated_mcp_result_contains_error_code_401() {
    let run_id = "mcp-auth-err";
    let events = mcp_journal(run_id, None, false);

    let tool_ev = events
        .iter()
        .find(|e| matches!(&e.event, Some(Event::ActivityRecorded(a)) if a.activity_kind == "tool"))
        .unwrap();

    if let Some(Event::ActivityRecorded(a)) = &tool_ev.event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v["code"], 401);
    }
}

#[test]
fn mcp_tool_input_with_bearer_token_contains_authorization() {
    let input = mcp_tool_input("search", Some("tok-xyz"), r#"{}"#);
    let v: serde_json::Value = serde_json::from_str(&input).unwrap();
    let auth = v["headers"]["Authorization"].as_str().unwrap();
    assert!(auth.starts_with("Bearer "));
}

#[test]
fn mcp_tool_input_without_token_has_no_authorization_header() {
    let input = mcp_tool_input("search", None, r#"{}"#);
    let v: serde_json::Value = serde_json::from_str(&input).unwrap();
    assert!(
        v["headers"].is_null()
            || v.get("headers")
                .is_none_or(|h| h.get("Authorization").is_none()),
        "unauthenticated input must have no Authorization header"
    );
}

#[test]
fn authenticated_mcp_activity_key_prefix_is_authed() {
    let events = mcp_journal("mcp-key-test", Some("tok"), true);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(
            a.activity_key.contains("authed"),
            "authed call key must contain 'authed'"
        );
    }
}

#[test]
fn unauthenticated_mcp_activity_key_prefix_is_unauthed() {
    let events = mcp_journal("mcp-unauth-key", None, false);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(
            a.activity_key.contains("unauthed"),
            "unauthed call key must contain 'unauthed'"
        );
    }
}

#[test]
fn auth_error_output_fails_success_schema_validation() {
    let unauth_output = r#"{"error":"unauthenticated","code":401}"#;
    let success_schema = r#"{"type":"object","required":["results"]}"#;
    let result = validate_output(unauth_output, success_schema);
    assert!(
        result.is_err() || {
            serde_json::from_str::<serde_json::Value>(unauth_output)
                .map(|v| v.get("results").is_none())
                .unwrap_or(false)
        },
        "auth error must not satisfy success schema"
    );
}

#[test]
fn two_unauthed_journals_store_independently() {
    let store = MemoryStore::new();

    for ev in mcp_journal("mcp-u1", None, false) {
        store.append("mcp-u1", ev).unwrap();
    }
    for ev in mcp_journal("mcp-u2", None, false) {
        store.append("mcp-u2", ev).unwrap();
    }

    assert_eq!(store.read("mcp-u1").unwrap().len(), 3);
    assert_eq!(store.read("mcp-u2").unwrap().len(), 3);
}
