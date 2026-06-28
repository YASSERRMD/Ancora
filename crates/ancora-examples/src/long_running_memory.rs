use ancora_memcon::{
    Turn, SummarizationPolicy, ConversationSummarizer,
    SalienceItem, SalienceScorer, EpisodicEntry, EpisodicToSemanticPromoter,
    ForgettingPolicy, ConsolidationJob, ConsolidationJournal,
    RetrievalChecker, TokenBudget,
};

pub fn run_long_running_memory_example() {
    let turns: Vec<Turn> = (0..10)
        .map(|i| Turn { index: i, role: "user".into(), content: format!("message {i}") })
        .collect();

    let episodic = vec![
        EpisodicEntry { key: "pref-rust".into(), content: "user prefers rust".into(), occurrences: 4 },
        EpisodicEntry { key: "pref-offline".into(), content: "user wants offline".into(), occurrences: 1 },
    ];

    let salience_items = vec![
        SalienceItem { key: "core-fact".into(), content: "rust async".into(), importance: 8, access_count: 5, age_secs: 10 },
        SalienceItem { key: "stale-fact".into(), content: "old noise".into(), importance: 1, access_count: 0, age_secs: 90_000 },
    ];

    let job = ConsolidationJob {
        summarizer: ConversationSummarizer::new(SummarizationPolicy::new(5, 2)),
        scorer: SalienceScorer::default_weights(),
        promoter: EpisodicToSemanticPromoter::new(3),
        forgetting: ForgettingPolicy::new(1.0, 50_000),
    };

    let mut journal = ConsolidationJournal::default();
    let out = job.run(&turns, salience_items, &episodic, 42, &mut journal);

    let retained_contents: Vec<String> = out.retained.iter().map(|i| i.content.clone()).collect();
    let quality_ok = RetrievalChecker::check(&retained_contents, &["rust"]);

    let _before_tokens = TokenBudget::total_tokens(
        &turns.iter().map(|t| t.content.clone()).collect::<Vec<_>>()
    );
    let _after_tokens = TokenBudget::total_tokens(&retained_contents);

    assert!(quality_ok, "retrieval quality check failed post-consolidation");
    assert!(!out.promoted.is_empty(), "expected at least one promotion");
    assert!(!journal.entries().is_empty(), "journal must record events");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_running_memory_example_runs() {
        run_long_running_memory_example();
    }
}
