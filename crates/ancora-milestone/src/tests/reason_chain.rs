use ancora_reason::{CitationStore, ReasoningEvent, ReasoningJournal};

#[test]
fn reason_chain_citations_and_journal() {
    let mut cs = CitationStore::new();
    let mut journal = ReasoningJournal::default();

    cs.add("earth orbits sun", "astronomy-textbook".to_string());
    journal.record(
        1,
        ReasoningEvent::CitationAdded {
            claim: "earth orbits sun".to_string(),
            citation: "astronomy-textbook".to_string(),
        },
    );

    assert!(cs.has_citations("earth orbits sun"));
    assert_eq!(journal.events().len(), 1);
}
