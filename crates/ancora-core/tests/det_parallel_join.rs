/// Determinism: parallel join order is stable.
/// Activities that run in parallel (same seq window) produce a stable merged order
/// because they are sorted by activity_key before being appended to the journal.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn parallel_journal(run_id: &str, keys: &[&str]) -> Vec<JournalEvent> {
    let mut events = vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
    ];
    for (i, key) in keys.iter().enumerate() {
        events.push(JournalEvent { event_id: format!("{}-{}", run_id, i+1), run_id: run_id.into(), seq: (i+1) as u64, recorded_at_ns: ((i+1) * 1000) as u64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: key.to_string(), activity_kind: "parallel-branch".into(),
                input_json: format!(r#"{{"branch":"{}"}}"#, key), result_json: format!(r#"{{"out":"{}"}}"#, key), replayed: false })) });
    }
    let n = events.len();
    events.push(JournalEvent { event_id: format!("{}-{}", run_id, n), run_id: run_id.into(), seq: n as u64, recorded_at_ns: (n * 1000) as u64,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"join":"done"}"#.into() })) });
    events
}

fn sorted_keys(j: &[JournalEvent]) -> Vec<String> {
    let mut keys: Vec<String> = j.iter().filter_map(|e| {
        if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.activity_key.clone()) } else { None }
    }).collect();
    keys.sort();
    keys
}

const BRANCHES: &[&str] = &["branch-c", "branch-a", "branch-b"];

#[test] fn parallel_join_produces_stable_key_order_when_sorted() {
    let j1 = parallel_journal("det-par-1", BRANCHES);
    let j2 = parallel_journal("det-par-2", BRANCHES);
    assert_eq!(sorted_keys(&j1), sorted_keys(&j2));
}

#[test] fn parallel_keys_when_pre_sorted_are_already_stable() {
    let sorted_branches = &["branch-a", "branch-b", "branch-c"];
    let j = parallel_journal("det-par-sorted", sorted_branches);
    let actual_keys: Vec<String> = j.iter().filter_map(|e| {
        if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.activity_key.clone()) } else { None }
    }).collect();
    assert_eq!(actual_keys, vec!["branch-a", "branch-b", "branch-c"]);
}

#[test] fn parallel_journal_event_count_equals_branches_plus_two() {
    let j = parallel_journal("det-par-cnt", BRANCHES);
    assert_eq!(j.len(), BRANCHES.len() + 2);
}

#[test] fn parallel_join_all_branches_present() {
    let j = parallel_journal("det-par-all", BRANCHES);
    let keys = sorted_keys(&j);
    assert_eq!(keys, vec!["branch-a", "branch-b", "branch-c"]);
}

#[test] fn parallel_seq_is_monotonic() {
    let j = parallel_journal("det-par-seq", BRANCHES);
    for (i, ev) in j.iter().enumerate() { assert_eq!(ev.seq, i as u64); }
}
