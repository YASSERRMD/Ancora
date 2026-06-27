/// Vespa conformance -- offline mock fixture, no live Vespa.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_vespa_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.to_string(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.to_string(),
            seq: 1,
            recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "retrieve-vespa".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"vespa ranking","top_k":2,"application":"my-app","schema":"doc","ranking_profile":"default"}"#.into(),
                result_json: r#"[{"fields":{"text":"Vespa hit A","docid":"id:doc::001"},"relevance":0.94,"source":"my-app"},{"fields":{"text":"Vespa hit B","docid":"id:doc::002"},"relevance":0.87,"source":"my-app"}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"vespa-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn vespa_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "vespa-run-151";
    for ev in &build_vespa_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn vespa_activity_key_contains_vespa() {
    let events = build_vespa_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("vespa"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn vespa_input_has_ranking_profile() {
    let events = build_vespa_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("ranking_profile"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn vespa_result_uses_relevance_and_fields() {
    let events = build_vespa_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("relevance"));
        assert!(a.result_json.contains("fields"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn vespa_result_has_two_hits() {
    let events = build_vespa_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn vespa_seq_monotonic() {
    let events = build_vespa_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
