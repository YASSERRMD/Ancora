/// Determinism: corrupted journal is detected.
/// Out-of-order seq numbers and duplicate event_ids indicate journal corruption.
use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};

fn is_seq_valid(events: &[JournalEvent]) -> bool {
    events.iter().enumerate().all(|(i, ev)| ev.seq == i as u64)
}

fn has_duplicate_event_ids(events: &[JournalEvent]) -> bool {
    let ids: std::collections::HashSet<&str> = events.iter().map(|e| e.event_id.as_str()).collect();
    ids.len() < events.len()
}

fn valid_journal(run_id: &str) -> Vec<JournalEvent> {
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
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
    ]
}

fn out_of_order_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.into(),
            seq: 1,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.into(),
            seq: 0,
            recorded_at_ns: 1_000,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
    ]
}

fn duplicate_id_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: "dup-id".into(),
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
            event_id: "dup-id".into(),
            run_id: run_id.into(),
            seq: 1,
            recorded_at_ns: 1_000,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
    ]
}

#[test]
fn valid_journal_passes_seq_check() {
    assert!(is_seq_valid(&valid_journal("det-cor-valid")));
}

#[test]
fn out_of_order_journal_fails_seq_check() {
    assert!(!is_seq_valid(&out_of_order_journal("det-cor-oor")));
}

#[test]
fn duplicate_id_journal_is_detected() {
    assert!(has_duplicate_event_ids(&duplicate_id_journal(
        "det-cor-dup"
    )));
}

#[test]
fn valid_journal_has_no_duplicate_ids() {
    assert!(!has_duplicate_event_ids(&valid_journal("det-cor-ndup")));
}

#[test]
fn empty_journal_passes_both_checks() {
    let empty: Vec<JournalEvent> = vec![];
    assert!(is_seq_valid(&empty));
    assert!(!has_duplicate_event_ids(&empty));
}
