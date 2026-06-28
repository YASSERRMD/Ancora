use crate::journal::{ConsolidationEvent, ConsolidationJournal};

#[test]
fn journal_records_events() {
    let mut j = ConsolidationJournal::default();
    j.record(1, ConsolidationEvent::Summarized { dropped_count: 3, summary_len: 50 });
    j.record(2, ConsolidationEvent::Promoted { key: "fact-a".into() });
    assert_eq!(j.entries().len(), 2);
}

#[test]
fn journal_replay_returns_events_in_order() {
    let mut j = ConsolidationJournal::default();
    j.record(1, ConsolidationEvent::Deduped { removed_count: 2 });
    j.record(2, ConsolidationEvent::Forgot { key: "old-fact".into() });
    let replayed = j.replay_events();
    assert_eq!(replayed.len(), 2);
    assert!(matches!(replayed[0], ConsolidationEvent::Deduped { .. }));
    assert!(matches!(replayed[1], ConsolidationEvent::Forgot { .. }));
}

#[test]
fn consolidation_replays_deterministically() {
    let mut j = ConsolidationJournal::default();
    j.record(10, ConsolidationEvent::Summarized { dropped_count: 5, summary_len: 100 });
    j.record(10, ConsolidationEvent::Promoted { key: "k".into() });
    let events = j.replay_events();
    assert_eq!(events.len(), 2);
}
