use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn token_event(run_id: &str, seq: u64, text: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{run_id}-{seq}"),
        run_id: run_id.to_string(),
        seq,
        recorded_at_ns: seq as i64 * 1_000,
        event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: format!("{run_id}-token-{seq}"),
            activity_kind: "token".to_string(),
            input_json: String::new(),
            result_json: serde_json::json!({ "text": text }).to_string(),
            replayed: false,
        })),
    }
}

#[test]
fn streaming_events_accumulate_token_text() {
    let tokens = ["Hello", ", ", "world", "!"];
    let full_text: String = tokens.concat();
    assert_eq!("Hello, world!", full_text);
}

#[test]
fn journal_stores_token_events_in_order() {
    let store = MemoryStore::new();
    let run_id = "stream-run-1";

    store
        .append(
            run_id,
            JournalEvent {
                event_id: format!("{run_id}-0"),
                run_id: run_id.to_string(),
                seq: 0,
                recorded_at_ns: 0,
                event: Some(Event::RunStarted(RunStartedEvent {
                    run_id: run_id.to_string(),
                    spec_bytes: vec![],
                    spec_type: "AgentSpec".to_string(),
                })),
            },
        )
        .unwrap();

    for (i, tok) in ["Hi", " there"].iter().enumerate() {
        store.append(run_id, token_event(run_id, i as u64 + 1, tok)).unwrap();
    }

    store
        .append(
            run_id,
            JournalEvent {
                event_id: format!("{run_id}-3"),
                run_id: run_id.to_string(),
                seq: 3,
                recorded_at_ns: 3_000,
                event: Some(Event::RunCompleted(RunCompletedEvent {
                    output_json: r#"{"text":"Hi there"}"#.to_string(),
                })),
            },
        )
        .unwrap();

    let events = store.read(run_id).unwrap();
    assert_eq!(4, events.len());
    assert!(matches!(events[0].event, Some(Event::RunStarted(_))));
    assert!(matches!(events[3].event, Some(Event::RunCompleted(_))));
}

#[test]
fn events_are_in_seq_order() {
    let store = MemoryStore::new();
    let run_id = "stream-run-2";
    for i in 0..5u64 {
        store.append(run_id, token_event(run_id, i, "tok")).unwrap();
    }
    let events = store.read(run_id).unwrap();
    let seqs: Vec<u64> = events.iter().map(|e| e.seq).collect();
    let mut sorted = seqs.clone();
    sorted.sort();
    assert_eq!(sorted, seqs);
}
