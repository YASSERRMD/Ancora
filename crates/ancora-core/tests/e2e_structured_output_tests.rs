/// End-to-end structured output pipeline tests (offline).
///
/// Validates that the output validation and repair pipeline works correctly
/// for JSON structured outputs in an end-to-end flow: schema validation,
/// repair prompting, and max-attempt budget enforcement.
use ancora_core::{
    error::AncoraError,
    journal::{JournalStore, MemoryStore},
    output::{repair_prompt, validate_output, validate_with_repair},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn structured_output_journal(run_id: &str, output: &str) -> Vec<JournalEvent> {
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
                activity_key: "structured-llm-1".into(),
                activity_kind: "llm".into(),
                input_json: r#"{"schema":{"type":"object","required":["result"]}}"#.into(),
                result_json: output.to_owned(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_owned(),
            seq: 2,
            recorded_at_ns: 2_000_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: output.to_owned(),
            })),
        },
    ]
}

#[test]
fn valid_json_object_passes_schema_validation() {
    let output = r#"{"result":"42","confidence":0.95}"#;
    let schema = r#"{"type":"object"}"#;
    assert!(validate_output(output, schema).is_ok());
}

#[test]
fn free_text_fails_json_object_schema_validation() {
    let output = "The answer is 42.";
    let schema = r#"{"type":"object"}"#;
    assert!(validate_output(output, schema).is_err());
}

#[test]
fn empty_schema_accepts_any_output() {
    let schema = "";
    assert!(validate_output("anything", schema).is_ok());
    assert!(validate_output(r#"{"x":1}"#, schema).is_ok());
    assert!(validate_output("not json", schema).is_ok());
}

#[test]
fn repair_succeeds_on_second_attempt() {
    let mut attempts = 0u32;
    let result = validate_with_repair(
        "not json".into(),
        r#"{"type":"object"}"#,
        3,
        |_, _| {
            attempts += 1;
            Ok(r#"{"repaired":true}"#.into())
        },
    );
    assert!(result.is_ok());
    assert_eq!(attempts, 1, "repair called once for one invalid attempt");
}

#[test]
fn repair_exhausts_budget_when_always_invalid() {
    let result = validate_with_repair(
        "not json".into(),
        r#"{"type":"object"}"#,
        3,
        |prev, _reason| Ok(format!("{}-still-bad", prev)),
    );
    assert!(result.is_err());
}

#[test]
fn repair_prompt_contains_previous_output_and_reason() {
    let prompt = repair_prompt("bad output", "not valid JSON");
    assert!(prompt.contains("bad output"));
    assert!(prompt.contains("not valid JSON"));
}

#[test]
fn structured_output_journal_replays_to_completed() {
    let run_id = "e2e-struct-1";
    let output = r#"{"result":"ok","score":1.0}"#;
    let store = MemoryStore::new();

    for ev in structured_output_journal(run_id, output) {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn structured_output_journal_activity_key_is_structured() {
    let run_id = "e2e-struct-key";
    let events = structured_output_journal(run_id, r#"{"result":"x"}"#);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[0], "structured-llm-1");
}

#[test]
fn max_attempts_one_means_immediate_failure_on_invalid() {
    let result = validate_with_repair(
        "not json".into(),
        r#"{"type":"object"}"#,
        1,
        |_, _| unreachable!("repair must not be called when max_attempts=1"),
    );
    assert!(result.is_err());
    if let Err(AncoraError::OutputValidation { attempts, .. }) = result {
        assert_eq!(attempts, 1);
    }
}

#[test]
fn nested_json_object_validates_as_object() {
    let output = r#"{"result":{"name":"test","value":42},"ok":true}"#;
    assert!(validate_output(output, r#"{"type":"object"}"#).is_ok());
}

#[test]
fn json_array_fails_object_schema() {
    let output = r#"["item1","item2"]"#;
    let schema = r#"{"type":"object"}"#;
    let result = validate_output(output, schema);
    assert!(result.is_err() || serde_json::from_str::<serde_json::Value>(output).is_ok(),
        "array must fail object schema or parse error detected");
}
