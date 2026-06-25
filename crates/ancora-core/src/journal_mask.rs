use ancora_proto::ancora::{journal_event::Event, JournalEvent};

/// A stripped representation of a journal event used for cross-language
/// structural comparison.
///
/// All model-generated content (activity results, token text, error messages)
/// is removed. Only the event kind and the run/node identifiers are kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedEvent {
    pub kind: &'static str,
    pub run_id: String,
    pub node_id: Option<String>,
    pub seq: u64,
}

/// Strip model-generated content from a slice of journal events and return
/// the structural skeleton as a sequence of `MaskedEvent`s.
///
/// Events without a recognised inner kind are silently dropped.
pub fn mask_events(events: &[JournalEvent]) -> Vec<MaskedEvent> {
    events
        .iter()
        .filter_map(|ev| {
            let kind = event_kind(ev.event.as_ref()?)?;
            let node_id = event_node_id(ev.event.as_ref()?);
            Some(MaskedEvent {
                kind,
                run_id: ev.run_id.clone(),
                node_id,
                seq: ev.seq,
            })
        })
        .collect()
}

/// Assert that two masked event sequences have the same structural shape.
///
/// "Same shape" means identical length, kind sequence, and node-id sequence.
/// Run IDs and sequence numbers are NOT compared because different bindings
/// generate independent run IDs.
///
/// Returns `Ok(())` on match, `Err(description)` on mismatch.
pub fn assert_structurally_equal(
    a: &[MaskedEvent],
    b: &[MaskedEvent],
) -> Result<(), String> {
    if a.len() != b.len() {
        return Err(format!(
            "event count mismatch: left={} right={}",
            a.len(),
            b.len()
        ));
    }
    for (i, (ea, eb)) in a.iter().zip(b.iter()).enumerate() {
        if ea.kind != eb.kind {
            return Err(format!(
                "kind mismatch at index {}: left={:?} right={:?}",
                i, ea.kind, eb.kind
            ));
        }
        if ea.node_id != eb.node_id {
            return Err(format!(
                "node_id mismatch at index {}: left={:?} right={:?}",
                i, ea.node_id, eb.node_id
            ));
        }
    }
    Ok(())
}

fn event_kind(ev: &Event) -> Option<&'static str> {
    match ev {
        Event::RunStarted(_) => Some("started"),
        Event::NodeEntered(_) => Some("node_entered"),
        Event::NodeExited(_) => Some("node_exited"),
        Event::ActivityRecorded(_) => Some("activity_recorded"),
        Event::HumanDecisionRequested(_) => Some("human_decision_requested"),
        Event::HumanDecisionReceived(_) => Some("human_decision_received"),
        Event::RunCompleted(_) => Some("completed"),
        Event::Error(_) => Some("error"),
        Event::RetryScheduled(_) => Some("retry_scheduled"),
        Event::RunCancelled(_) => Some("run_cancelled"),
    }
}

fn event_node_id(ev: &Event) -> Option<String> {
    match ev {
        Event::NodeEntered(e) => Some(e.node_id.clone()),
        Event::NodeExited(e) => Some(e.node_id.clone()),
        Event::ActivityRecorded(e) => Some(e.activity_key.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ancora_proto::ancora::{
        journal_event::Event, ActivityRecordedEvent, JournalEvent, NodeEnteredEvent,
        RunCompletedEvent, RunStartedEvent,
    };

    fn make_event(seq: u64, run_id: &str, ev: Event) -> JournalEvent {
        JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: 0,
            event: Some(ev),
        }
    }

    fn started(seq: u64, run_id: &str) -> JournalEvent {
        make_event(seq, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        }))
    }

    fn completed(seq: u64, run_id: &str) -> JournalEvent {
        make_event(seq, run_id, Event::RunCompleted(RunCompletedEvent {
            output_json: String::new(),
        }))
    }

    fn node_entered(seq: u64, run_id: &str, node: &str) -> JournalEvent {
        make_event(seq, run_id, Event::NodeEntered(NodeEnteredEvent {
            node_id: node.to_owned(),
            node_kind: "agent".into(),
        }))
    }

    fn activity(seq: u64, run_id: &str, key: &str) -> JournalEvent {
        make_event(seq, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key.into(),
            activity_kind: "llm_call".into(),
            input_json: "{}".into(),
            result_json: "model generated text -- stripped by mask".into(),
            replayed: false,
        }))
    }

    #[test]
    fn mask_strips_model_generated_content() {
        let events = vec![started(0, "r1"), activity(1, "r1", "act-key-1"), completed(2, "r1")];
        let masked = mask_events(&events);
        assert_eq!(masked.len(), 3);
        assert_eq!(masked[0].kind, "started");
        assert_eq!(masked[1].kind, "activity_recorded");
        assert_eq!(masked[2].kind, "completed");
        assert_eq!(masked[1].node_id, Some("act-key-1".into()));
        assert_eq!(masked[0].node_id, None);
    }

    #[test]
    fn structurally_equal_same_sequence() {
        let a = vec![started(0, "r1"), completed(1, "r1")];
        let b = vec![started(0, "r2"), completed(1, "r2")];
        let ma = mask_events(&a);
        let mb = mask_events(&b);
        assert!(assert_structurally_equal(&ma, &mb).is_ok());
    }

    #[test]
    fn structurally_unequal_different_length() {
        let a = vec![started(0, "r1"), completed(1, "r1")];
        let b = vec![started(0, "r2")];
        let ma = mask_events(&a);
        let mb = mask_events(&b);
        let err = assert_structurally_equal(&ma, &mb).unwrap_err();
        assert!(err.contains("count mismatch"));
    }

    #[test]
    fn structurally_unequal_different_kinds() {
        let a = vec![started(0, "r1"), completed(1, "r1")];
        let b = vec![started(0, "r2"), node_entered(1, "r2", "n")];
        let ma = mask_events(&a);
        let mb = mask_events(&b);
        let err = assert_structurally_equal(&ma, &mb).unwrap_err();
        assert!(err.contains("kind mismatch"));
    }

    #[test]
    fn structurally_unequal_different_node_ids() {
        let a = vec![node_entered(0, "r1", "node-a"), completed(1, "r1")];
        let b = vec![node_entered(0, "r2", "node-b"), completed(1, "r2")];
        let ma = mask_events(&a);
        let mb = mask_events(&b);
        let err = assert_structurally_equal(&ma, &mb).unwrap_err();
        assert!(err.contains("node_id mismatch"), "expected node_id mismatch, got: {}", err);
    }

    #[test]
    fn run_ids_are_not_compared() {
        let a = vec![started(0, "run-aaa"), completed(1, "run-aaa")];
        let b = vec![started(0, "run-zzz"), completed(1, "run-zzz")];
        let ma = mask_events(&a);
        let mb = mask_events(&b);
        assert!(assert_structurally_equal(&ma, &mb).is_ok(), "run IDs should not affect structural equality");
    }

    #[test]
    fn events_without_inner_kind_are_dropped() {
        let mut ev = started(0, "r1");
        ev.event = None;
        let masked = mask_events(&[ev]);
        assert!(masked.is_empty());
    }
}
