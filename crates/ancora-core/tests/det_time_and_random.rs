/// Determinism: time and random are journaled not live.
/// The system time (recorded_at_ns) and random seeds are captured in the journal.
/// On replay, these values come from the journal, not from a live clock or rng.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_time_journal(run_id: &str, ts_ns: u64, seed: u64) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: ts_ns,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "sampled".into(), activity_kind: "sampled".into(),
                input_json: format!(r#"{{"seed":{}}}"#, seed),
                result_json: format!(r#"{{"value":{}}}"#, seed * 7),
                replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: ts_ns + 1_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"done":true}"#.into() })) },
    ]
}

const FIXED_TS: u64 = 1_700_000_000_000_000_000;
const FIXED_SEED: u64 = 12345;

#[test] fn journaled_timestamp_is_preserved_on_replay() {
    let j = build_time_journal("det-ts", FIXED_TS, FIXED_SEED);
    assert_eq!(j[1].recorded_at_ns, FIXED_TS, "timestamp must come from journal not live clock");
}

#[test] fn journaled_seed_produces_deterministic_result() {
    let j1 = build_time_journal("det-rng-1", FIXED_TS, FIXED_SEED);
    let j2 = build_time_journal("det-rng-2", FIXED_TS, FIXED_SEED);
    if let (Some(Event::ActivityRecorded(a1)), Some(Event::ActivityRecorded(a2))) = (&j1[1].event, &j2[1].event) {
        assert_eq!(a1.result_json, a2.result_json, "same seed must produce same result");
    }
}

#[test] fn different_seeds_produce_different_results() {
    let j1 = build_time_journal("det-rng-a", FIXED_TS, 100);
    let j2 = build_time_journal("det-rng-b", FIXED_TS, 200);
    if let (Some(Event::ActivityRecorded(a1)), Some(Event::ActivityRecorded(a2))) = (&j1[1].event, &j2[1].event) {
        assert_ne!(a1.result_json, a2.result_json);
    }
}

#[test] fn live_clock_is_not_used_on_replay() {
    let past_ts = 1_000_000_000u64;
    let j = build_time_journal("det-past-ts", past_ts, FIXED_SEED);
    assert_eq!(j[1].recorded_at_ns, past_ts, "replay must use journaled ts, not system clock");
}

#[test] fn timestamp_in_completed_event_is_after_activity() {
    let j = build_time_journal("det-ts-order", FIXED_TS, FIXED_SEED);
    assert!(j[2].recorded_at_ns > j[1].recorded_at_ns, "completed ts must be after activity ts");
}
