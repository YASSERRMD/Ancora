/// In-memory vector store conformance -- offline, no live store.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_inmemory_journal(run_id: &str, chunks: &[(&str, f32)]) -> Vec<JournalEvent> {
    let result: String = {
        let items: Vec<String> = chunks
            .iter()
            .map(|(t, s)| {
                format!(
                    r#"{{"text":{},"score":{}}}"#,
                    serde_json::to_string(t).unwrap(),
                    s
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    };
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
                activity_key: "retrieve-inmemory".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"test","top_k":3}"#.into(),
                result_json: result,
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"ok"}"#.into(),
            })),
        },
    ]
}

const CHUNKS: &[(&str, f32)] = &[
    ("In-memory chunk A", 0.97),
    ("In-memory chunk B", 0.91),
    ("In-memory chunk C", 0.85),
];

#[test]
fn inmemory_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "inmem-run-151";
    for ev in &build_inmemory_journal(run_id, CHUNKS) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn inmemory_journal_has_three_events() {
    let events = build_inmemory_journal("r1", CHUNKS);
    assert_eq!(events.len(), 3);
}

#[test]
fn inmemory_first_event_is_run_started() {
    let events = build_inmemory_journal("r1", CHUNKS);
    assert!(matches!(events[0].event, Some(Event::RunStarted(_))));
}

#[test]
fn inmemory_last_event_is_run_completed() {
    let events = build_inmemory_journal("r1", CHUNKS);
    assert!(matches!(events[2].event, Some(Event::RunCompleted(_))));
}

#[test]
fn inmemory_retrieval_has_three_chunks() {
    let events = build_inmemory_journal("r1", CHUNKS);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn inmemory_activity_kind_is_retrieval() {
    let events = build_inmemory_journal("r1", CHUNKS);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert_eq!(a.activity_kind, "retrieval");
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn inmemory_seq_is_monotonic() {
    let events = build_inmemory_journal("r1", CHUNKS);
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}

#[test]
fn inmemory_run_ids_all_match() {
    let run_id = "inmem-match";
    for ev in &build_inmemory_journal(run_id, CHUNKS) {
        assert_eq!(ev.run_id, run_id);
    }
}
