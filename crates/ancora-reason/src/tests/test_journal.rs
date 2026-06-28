use crate::journal::{ReasoningEvent, ReasoningJournal};

#[test]
fn reasoning_replays_deterministically() {
    let mut journal = ReasoningJournal::default();
    journal.record(1, ReasoningEvent::StepAdded { index: 0, claim: "A".into() });
    journal.record(2, ReasoningEvent::StepVerified { index: 0 });
    journal.record(3, ReasoningEvent::ContradictionFound { a: 0, b: 1 });

    let replayed = journal.replay();
    assert_eq!(replayed.len(), 3);
    assert_eq!(replayed[0], &ReasoningEvent::StepAdded { index: 0, claim: "A".into() });
    assert_eq!(replayed[1], &ReasoningEvent::StepVerified { index: 0 });
    assert_eq!(replayed[2], &ReasoningEvent::ContradictionFound { a: 0, b: 1 });
}

#[test]
fn journal_records_all_event_types() {
    let mut journal = ReasoningJournal::default();
    journal.record(1, ReasoningEvent::StepAdded { index: 0, claim: "c".into() });
    journal.record(2, ReasoningEvent::StepRefuted { index: 0 });
    journal.record(3, ReasoningEvent::StepAbstained { index: 1 });
    journal.record(4, ReasoningEvent::FactChecked { claim: "c".into(), grounded: true });
    journal.record(5, ReasoningEvent::CitationAdded { claim: "c".into(), citation: "src".into() });
    assert_eq!(journal.events().len(), 5);
}

#[test]
fn journal_preserves_tick_order() {
    let mut journal = ReasoningJournal::default();
    journal.record(10, ReasoningEvent::StepVerified { index: 0 });
    journal.record(20, ReasoningEvent::StepVerified { index: 1 });
    let events = journal.events();
    assert_eq!(events[0].0, 10);
    assert_eq!(events[1].0, 20);
}
