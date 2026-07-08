use ancora_reason::{
    CitationStore, ContradictionDetector, ReasoningEvent, ReasoningJournal, StepDecomposer,
    StepVerifier,
};

fn run_reasoning(journal: &mut ReasoningJournal) {
    let mut steps = StepDecomposer::decompose(vec![
        "water boils at 100C".into(),
        "NOT: water boils at 100C".into(),
        "ice melts at 0C".into(),
    ]);

    for step in steps.iter_mut() {
        let result = StepVerifier::verify(step, |c| !c.starts_with("NOT:"));
        let event = if result.passed {
            ReasoningEvent::StepVerified { index: step.index }
        } else {
            ReasoningEvent::StepRefuted { index: step.index }
        };
        journal.record(1, event);
    }

    let contradictions = ContradictionDetector::detect(&steps);
    for (a, b) in contradictions {
        journal.record(2, ReasoningEvent::ContradictionFound { a, b });
    }
}

#[test]
fn reason_journal_replay_event_count_stable() {
    let mut j1 = ReasoningJournal::default();
    let mut j2 = ReasoningJournal::default();
    run_reasoning(&mut j1);
    run_reasoning(&mut j2);
    assert_eq!(j1.events().len(), j2.events().len());
}

#[test]
fn reason_journal_replay_events_stable() {
    let mut j1 = ReasoningJournal::default();
    let mut j2 = ReasoningJournal::default();
    run_reasoning(&mut j1);
    run_reasoning(&mut j2);
    let r1 = j1.replay();
    let r2 = j2.replay();
    assert_eq!(r1.len(), r2.len());
    for (a, b) in r1.iter().zip(r2.iter()) {
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }
}

#[test]
fn reason_citation_store_stable() {
    let mut cs1 = CitationStore::default();
    let mut cs2 = CitationStore::default();

    cs1.add("ice melts at 0C", "physics-textbook".to_string());
    cs2.add("ice melts at 0C", "physics-textbook".to_string());

    assert_eq!(
        cs1.has_citations("ice melts at 0C"),
        cs2.has_citations("ice melts at 0C")
    );
    assert_eq!(cs1.get("ice melts at 0C"), cs2.get("ice melts at 0C"));
}
