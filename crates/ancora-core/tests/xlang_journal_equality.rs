use ancora_core::journal_mask::{assert_structurally_equal, mask_events};
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, NodeEnteredEvent, RunCompletedEvent, RunStartedEvent,
};

fn run_started(seq: u64, run_id: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }
}

fn node_entered(seq: u64, run_id: &str, node: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::NodeEntered(NodeEnteredEvent {
            node_id: node.to_owned(),
            node_kind: "agent".into(),
        })),
    }
}

fn run_completed(seq: u64, run_id: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: String::new(),
        })),
    }
}

fn single_agent_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        run_started(0, run_id),
        node_entered(1, run_id, "agent-node"),
        run_completed(2, run_id),
    ]
}

#[test]
fn single_agent_journals_from_two_bindings_are_structurally_equal() {
    let rust_journal = single_agent_journal("rust-run-abc");
    let go_journal = single_agent_journal("go-run-xyz");

    let masked_rust = mask_events(&rust_journal);
    let masked_go = mask_events(&go_journal);

    assert_structurally_equal(&masked_rust, &masked_go)
        .expect("rust and go journals should be structurally identical");
}

#[test]
fn journals_with_different_model_text_are_still_equal() {
    use ancora_proto::ancora::ActivityRecordedEvent;

    fn with_activity(run_id: &str, output: &str) -> Vec<JournalEvent> {
        vec![
            run_started(0, run_id),
            JournalEvent {
                event_id: format!("{}-1", run_id),
                run_id: run_id.to_owned(),
                seq: 1,
                recorded_at_ns: 0,
                event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                    activity_key: "llm-call-1".into(),
                    activity_kind: "llm".into(),
                    input_json: "{}".into(),
                    result_json: output.to_owned(),
                    replayed: false,
                })),
            },
            run_completed(2, run_id),
        ]
    }

    let journal_a = with_activity("r-a", r#"{"text":"Hello from Python"}"#);
    let journal_b = with_activity("r-b", r#"{"text":"Hola from TypeScript"}"#);

    let ma = mask_events(&journal_a);
    let mb = mask_events(&journal_b);

    assert_structurally_equal(&ma, &mb)
        .expect("journals should be structurally equal regardless of model text");
}

#[test]
fn journals_with_different_event_counts_are_unequal() {
    let short = vec![run_started(0, "r1"), run_completed(1, "r1")];
    let long = vec![
        run_started(0, "r2"),
        node_entered(1, "r2", "agent"),
        run_completed(2, "r2"),
    ];

    let ms = mask_events(&short);
    let ml = mask_events(&long);

    assert!(
        assert_structurally_equal(&ms, &ml).is_err(),
        "journals with different event counts must not compare equal"
    );
}

#[test]
fn journals_with_different_node_ids_are_unequal() {
    let a = vec![node_entered(0, "r1", "node-alpha"), run_completed(1, "r1")];
    let b = vec![node_entered(0, "r2", "node-beta"), run_completed(1, "r2")];

    let ma = mask_events(&a);
    let mb = mask_events(&b);

    assert!(
        assert_structurally_equal(&ma, &mb).is_err(),
        "journals diverging on node_id must not compare equal"
    );
}

#[test]
fn empty_journals_are_equal() {
    let ma = mask_events(&[]);
    let mb = mask_events(&[]);
    assert!(assert_structurally_equal(&ma, &mb).is_ok());
}
