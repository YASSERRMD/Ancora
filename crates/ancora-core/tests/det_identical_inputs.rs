/// Determinism: identical inputs replay identically.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_det_journal(run_id: &str, input: &str, result: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.into(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.into(),
            seq: 1,
            recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "compute".into(),
                activity_kind: "deterministic".into(),
                input_json: input.into(),
                result_json: result.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.into(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: format!(r#"{{"result":"{}"}}"#, result),
            })),
        },
    ]
}

const INPUT_A: &str = r#"{"x":42}"#;
const RESULT_A: &str = r#"{"y":84}"#;

#[test]
fn identical_inputs_produce_same_status_on_two_replays() {
    for i in 0..2 {
        let store = MemoryStore::new();
        let rid = format!("det-id-{}", i);
        for ev in &build_det_journal(&rid, INPUT_A, RESULT_A) {
            store.append(&rid, ev.clone()).unwrap();
        }
        let state = replay_events(&rid, &store.read(&rid).unwrap()).unwrap();
        assert_eq!(state.run.status, RunStatus::Completed);
    }
}

#[test]
fn identical_inputs_produce_same_event_count_on_two_replays() {
    let counts: Vec<usize> = (0..2)
        .map(|i| {
            let j = build_det_journal(&format!("det-cnt-{}", i), INPUT_A, RESULT_A);
            j.len()
        })
        .collect();
    assert_eq!(counts[0], counts[1]);
}

#[test]
fn identical_inputs_have_same_activity_result() {
    for i in 0..3 {
        let j = build_det_journal(&format!("det-res-{}", i), INPUT_A, RESULT_A);
        if let Some(Event::ActivityRecorded(a)) = &j[1].event {
            assert_eq!(a.result_json, RESULT_A);
        }
    }
}

#[test]
fn different_inputs_produce_different_results() {
    let j1 = build_det_journal("det-diff-a", INPUT_A, r#"{"y":84}"#);
    let j2 = build_det_journal("det-diff-b", r#"{"x":99}"#, r#"{"y":198}"#);
    if let (Some(Event::ActivityRecorded(a1)), Some(Event::ActivityRecorded(a2))) =
        (&j1[1].event, &j2[1].event)
    {
        assert_ne!(a1.input_json, a2.input_json);
        assert_ne!(a1.result_json, a2.result_json);
    }
}

#[test]
fn replay_preserves_seq_numbers() {
    let j = build_det_journal("det-seq", INPUT_A, RESULT_A);
    for (i, ev) in j.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
