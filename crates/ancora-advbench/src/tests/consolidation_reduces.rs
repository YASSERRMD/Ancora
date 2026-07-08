use ancora_memcon::{
    ConsolidationJob, ConsolidationJournal, ConversationSummarizer, EpisodicEntry,
    EpisodicToSemanticPromoter, ForgettingPolicy, SalienceItem, SalienceScorer,
    SummarizationPolicy, Turn,
};

fn make_job(min_occ: u32) -> ConsolidationJob {
    ConsolidationJob {
        summarizer: ConversationSummarizer::new(SummarizationPolicy::new(5, 2)),
        scorer: SalienceScorer::default_weights(),
        promoter: EpisodicToSemanticPromoter::new(min_occ),
        forgetting: ForgettingPolicy::new(0.0, u64::MAX),
    }
}

fn dummy_turns() -> Vec<Turn> {
    (0..2)
        .map(|i| Turn {
            index: i,
            role: "user".into(),
            content: format!("t{i}"),
        })
        .collect()
}

#[test]
fn consolidation_promotes_entries() {
    let job = make_job(1);
    let turns = dummy_turns();
    let episodic: Vec<EpisodicEntry> = (0u32..20)
        .map(|i| EpisodicEntry {
            key: format!("k{i}"),
            content: format!("c{i}"),
            occurrences: 2,
        })
        .collect();
    let mut journal = ConsolidationJournal::default();
    let output = job.run(&turns, vec![], &episodic, 1, &mut journal);
    assert!(
        !output.promoted.is_empty(),
        "consolidation should promote at least one entry"
    );
}

#[test]
fn consolidation_token_units_less_than_input() {
    let job = make_job(3);
    let turns = dummy_turns();
    let episodic: Vec<EpisodicEntry> = (0u32..50)
        .map(|i| EpisodicEntry {
            key: format!("k{i}"),
            content: format!("c{i}"),
            occurrences: if i % 4 == 0 { 4 } else { 1 },
        })
        .collect();
    let salience: Vec<SalienceItem> = vec![];
    let mut journal = ConsolidationJournal::default();
    let output = job.run(&turns, salience, &episodic, 1, &mut journal);
    assert!(
        output.promoted.len() < 50,
        "promoted ({}) should be fewer than input (50)",
        output.promoted.len()
    );
}
