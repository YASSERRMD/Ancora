use ancora_examples::RunJournal;
use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn start_event(run_id: &str) -> JournalEvent {
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
    }
}

fn complete_event(run_id: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{run_id}-1"),
        run_id: run_id.to_string(),
        seq: 1,
        recorded_at_ns: 1_000,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: r#"{"result":"ok"}"#.to_string(),
        })),
    }
}

#[test]
fn run_journal_records_and_replays() {
    let mut journal = RunJournal::new();
    journal.record_run("run-1");
    journal.append_event("run-1", r#"{"kind":"started"}"#);
    journal.append_event("run-1", r#"{"kind":"completed"}"#);

    assert_eq!(2, journal.events_for_run("run-1").len());
    assert_eq!(1, journal.run_count());
}

#[test]
fn run_journal_returns_empty_slice_for_unknown_run() {
    let journal = RunJournal::new();
    assert!(journal.events_for_run("missing").is_empty());
}

#[test]
fn run_journal_record_run_is_idempotent() {
    let mut journal = RunJournal::new();
    journal.record_run("dup");
    journal.record_run("dup");
    assert_eq!(1, journal.run_count());
}

#[test]
fn run_journal_tracks_multiple_runs() {
    let mut journal = RunJournal::new();
    journal.record_run("a");
    journal.record_run("b");
    assert_eq!(2, journal.run_count());
}

#[test]
fn memory_store_persists_events_across_reads() {
    let store = MemoryStore::new();
    let run_id = "restart-run";
    store.append(run_id, start_event(run_id)).unwrap();
    store.append(run_id, complete_event(run_id)).unwrap();

    let replayed = store.read(run_id).unwrap();
    assert_eq!(2, replayed.len());
    assert!(matches!(replayed[0].event, Some(Event::RunStarted(_))));
    assert!(matches!(replayed[1].event, Some(Event::RunCompleted(_))));
}

#[test]
fn memory_store_is_independent_per_run() {
    let store = MemoryStore::new();
    store.append("run-a", start_event("run-a")).unwrap();
    store.append("run-b", start_event("run-b")).unwrap();

    assert_eq!(1, store.read("run-a").unwrap().len());
    assert_eq!(1, store.read("run-b").unwrap().len());
    assert!(store.read("run-c").unwrap().is_empty());
}
