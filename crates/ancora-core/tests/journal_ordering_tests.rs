use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};

fn make_event(run_id: &str, seq: u64) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![seq as u8],
            spec_type: "AgentSpec".into(),
        })),
    }
}

#[test]
fn append_single_event_assigns_seq_zero() {
    let store = MemoryStore::new();
    let seq = store.append("run-a", make_event("run-a", 0)).unwrap();
    assert_eq!(seq, 0, "first append must get seq 0");
}

#[test]
fn append_multiple_events_sequences_ascend() {
    let store = MemoryStore::new();
    for i in 0..5u64 {
        let seq = store.append("run-b", make_event("run-b", i)).unwrap();
        assert_eq!(seq, i, "seq must match insertion order");
    }
}

#[test]
fn read_returns_events_in_seq_order() {
    let store = MemoryStore::new();
    for i in 0..4u64 {
        store.append("run-c", make_event("run-c", i)).unwrap();
    }
    let events = store.read("run-c").unwrap();
    let seqs: Vec<u64> = events.iter().map(|e| e.seq).collect();
    assert_eq!(seqs, vec![0, 1, 2, 3], "events must be in ascending seq order");
}

#[test]
fn read_empty_run_returns_empty_vec() {
    let store = MemoryStore::new();
    let events = store.read("nonexistent").unwrap();
    assert!(events.is_empty(), "unknown run must return empty vec");
}

#[test]
fn append_to_different_runs_does_not_mix() {
    let store = MemoryStore::new();
    store.append("run-x", make_event("run-x", 0)).unwrap();
    store.append("run-y", make_event("run-y", 0)).unwrap();
    let x = store.read("run-x").unwrap();
    let y = store.read("run-y").unwrap();
    assert_eq!(x.len(), 1);
    assert_eq!(y.len(), 1);
    assert_eq!(x[0].run_id, "run-x");
    assert_eq!(y[0].run_id, "run-y");
}

#[test]
fn load_by_seq_returns_correct_event() {
    let store = MemoryStore::new();
    for i in 0..3u64 {
        store.append("run-d", make_event("run-d", i)).unwrap();
    }
    let ev = store.load("run-d", 1).unwrap();
    assert!(ev.is_some(), "seq 1 must be found");
    assert_eq!(ev.unwrap().seq, 1);
}

#[test]
fn load_past_end_returns_none() {
    let store = MemoryStore::new();
    store.append("run-e", make_event("run-e", 0)).unwrap();
    let ev = store.load("run-e", 99).unwrap();
    assert!(ev.is_none(), "seq beyond last must return None");
}

#[test]
fn append_large_batch_preserves_all_events() {
    let store = MemoryStore::new();
    let n = 1000u64;
    for i in 0..n {
        store.append("run-big", make_event("run-big", i)).unwrap();
    }
    let events = store.read("run-big").unwrap();
    assert_eq!(events.len() as u64, n, "all {n} events must be present");
}
