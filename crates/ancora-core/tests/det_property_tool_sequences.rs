/// Determinism: property test -- random tool sequences replay correctly.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

const TOOLS: &[&str] = &["search", "calculate", "fetch", "summarise", "classify", "translate"];

fn lcg_next(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}

fn tool_sequence_journal(run_id: &str, length: usize, seed: u64) -> Vec<JournalEvent> {
    let mut rng = seed;
    let mut events = vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
    ];
    for i in 0..length {
        let tool = TOOLS[(lcg_next(&mut rng) as usize) % TOOLS.len()];
        let result_val = lcg_next(&mut rng) % 1000;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i+1), run_id: run_id.into(), seq: (i+1) as u64, recorded_at_ns: ((i+1)*1000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("tool:{}", tool), activity_kind: "tool-call".into(),
                input_json: format!(r#"{{"tool":"{}","step":{}}}"#, tool, i),
                result_json: format!(r#"{{"tool":"{}","value":{}}}"#, tool, result_val), replayed: false })) });
    }
    let n = events.len();
    events.push(JournalEvent { event_id: format!("{}-{}", run_id, n), run_id: run_id.into(), seq: n as u64, recorded_at_ns: (n*1000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"ok":true}"#.into() })) });
    events
}

const SEQUENCES: &[(usize, u64)] = &[(1,10),(3,20),(6,30),(9,40),(12,50),(5,99),(8,777)];

#[test] fn all_tool_sequences_replay_to_completed() {
    for (len, seed) in SEQUENCES {
        let rid = format!("det-tool-{}-{}", len, seed);
        let j = tool_sequence_journal(&rid, *len, *seed);
        let store = MemoryStore::new();
        for ev in &j { store.append(&rid, ev.clone()).unwrap(); }
        let status = replay_events(&rid, &store.read(&rid).unwrap()).unwrap().run.status;
        assert_eq!(status, RunStatus::Completed, "failed for len={} seed={}", len, seed);
    }
}

#[test] fn tool_sequence_event_count_equals_length_plus_two() {
    for (len, seed) in SEQUENCES {
        let j = tool_sequence_journal(&format!("det-tcnt-{}", seed), *len, *seed);
        assert_eq!(j.len(), len + 2);
    }
}

#[test] fn same_seed_produces_same_tool_sequence() {
    let j1 = tool_sequence_journal("det-tsame-1", 5, 42);
    let j2 = tool_sequence_journal("det-tsame-2", 5, 42);
    let keys1: Vec<String> = j1.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.activity_key.clone()) } else { None }).collect();
    let keys2: Vec<String> = j2.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.activity_key.clone()) } else { None }).collect();
    assert_eq!(keys1, keys2);
}

#[test] fn all_tool_activity_keys_start_with_tool_prefix() {
    let j = tool_sequence_journal("det-tkeys", 6, 99);
    for ev in &j {
        if let Some(Event::ActivityRecorded(a)) = &ev.event {
            assert!(a.activity_key.starts_with("tool:"), "expected tool: prefix, got {}", a.activity_key);
        }
    }
}

#[test] fn tool_sequence_seq_monotonic() {
    let j = tool_sequence_journal("det-tseq", 5, 42);
    for (i, ev) in j.iter().enumerate() { assert_eq!(ev.seq, i as u64); }
}
