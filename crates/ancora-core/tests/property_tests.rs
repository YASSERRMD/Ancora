use ancora_core::replay::replay_events;
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, RunCompletedEvent, RunStartedEvent,
};
use proptest::prelude::*;

fn make_event(seq: u64, run_id: &str, ev: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(ev),
    }
}

fn started_event(seq: u64, run_id: &str) -> JournalEvent {
    make_event(seq, run_id, Event::RunStarted(RunStartedEvent {
        run_id: run_id.to_owned(),
        spec_bytes: vec![],
        spec_type: "AgentSpec".into(),
    }))
}

fn completed_event(seq: u64, run_id: &str) -> JournalEvent {
    make_event(seq, run_id, Event::RunCompleted(RunCompletedEvent {
        output_json: String::new(),
    }))
}

proptest! {
    #[test]
    fn replay_is_deterministic_for_any_run_id(
        run_id in "[a-z][a-z0-9-]{1,16}"
    ) {
        let events = vec![started_event(0, &run_id), completed_event(1, &run_id)];
        let result_a = replay_events(&run_id, &events).unwrap();
        let result_b = replay_events(&run_id, &events).unwrap();

        prop_assert_eq!(
            format!("{:?}", result_a.run.status),
            format!("{:?}", result_b.run.status),
            "replay must be deterministic"
        );
    }

    #[test]
    fn replay_of_empty_event_list_always_produces_pending_run(
        run_id in "[a-z]{3,10}"
    ) {
        let state = replay_events(&run_id, &[]).unwrap();
        prop_assert_eq!(
            format!("{:?}", state.run.status),
            "Pending",
            "empty journal must produce Pending run"
        );
    }

    #[test]
    fn replay_activity_keys_match_insertion_order(
        keys in proptest::collection::vec("[a-z]{3,8}", 1..8)
    ) {
        use ancora_proto::ancora::ActivityRecordedEvent;

        let run_id = "prop-run";
        let mut events = vec![started_event(0, run_id)];
        for (i, key) in keys.iter().enumerate() {
            events.push(make_event(
                (i + 1) as u64,
                run_id,
                Event::ActivityRecorded(ActivityRecordedEvent {
                    activity_key: key.clone(),
                    activity_kind: "llm".into(),
                    input_json: "{}".into(),
                    result_json: "{}".into(),
                    replayed: false,
                }),
            ));
        }
        events.push(completed_event((keys.len() + 1) as u64, run_id));

        let state = replay_events(run_id, &events).unwrap();
        prop_assert_eq!(
            state.activity_keys,
            keys,
            "activity key order must be preserved across replay"
        );
    }

    #[test]
    fn replaying_same_events_twice_gives_same_activity_key_count(
        n in 0usize..10
    ) {
        use ancora_proto::ancora::ActivityRecordedEvent;

        let run_id = "prop-idem";
        let mut events = vec![started_event(0, run_id)];
        for i in 0..n {
            events.push(make_event(
                (i + 1) as u64,
                run_id,
                Event::ActivityRecorded(ActivityRecordedEvent {
                    activity_key: format!("key-{}", i),
                    activity_kind: "llm".into(),
                    input_json: "{}".into(),
                    result_json: "{}".into(),
                    replayed: false,
                }),
            ));
        }
        events.push(completed_event((n + 1) as u64, run_id));

        let a = replay_events(run_id, &events).unwrap();
        let b = replay_events(run_id, &events).unwrap();

        prop_assert_eq!(a.activity_keys.len(), b.activity_keys.len());
    }
}
