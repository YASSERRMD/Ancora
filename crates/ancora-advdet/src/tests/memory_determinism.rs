use ancora_memcon::{
    ConsolidationJob, ConsolidationJournal, ConversationSummarizer, EpisodicEntry,
    EpisodicToSemanticPromoter, ForgettingPolicy, SalienceItem, SalienceScorer,
    SummarizationPolicy, Turn,
};

fn make_job() -> ConsolidationJob {
    ConsolidationJob {
        summarizer: ConversationSummarizer::new(SummarizationPolicy::new(3, 1)),
        scorer: SalienceScorer::default_weights(),
        promoter: EpisodicToSemanticPromoter::new(1),
        forgetting: ForgettingPolicy::new(0.0, 9999),
    }
}

fn sample_turns() -> Vec<Turn> {
    vec![
        Turn {
            index: 0,
            role: "user".into(),
            content: "hello".into(),
        },
        Turn {
            index: 1,
            role: "assistant".into(),
            content: "hi".into(),
        },
        Turn {
            index: 2,
            role: "user".into(),
            content: "what is Rust?".into(),
        },
        Turn {
            index: 3,
            role: "assistant".into(),
            content: "a systems language".into(),
        },
    ]
}

fn sample_salience() -> Vec<SalienceItem> {
    vec![
        SalienceItem {
            key: "k1".into(),
            content: "abc".into(),
            importance: 2,
            access_count: 1,
            age_secs: 10,
        },
        SalienceItem {
            key: "k2".into(),
            content: "xyz".into(),
            importance: 1,
            access_count: 0,
            age_secs: 20,
        },
    ]
}

fn sample_episodic() -> Vec<EpisodicEntry> {
    vec![EpisodicEntry {
        key: "e1".into(),
        content: "user asked about Rust".into(),
        occurrences: 2,
    }]
}

#[test]
fn memory_consolidation_journal_replay_stable() {
    let job = make_job();
    let mut j1 = ConsolidationJournal::default();
    let mut j2 = ConsolidationJournal::default();

    job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j1,
    );
    job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j2,
    );

    assert_eq!(j1.entries().len(), j2.entries().len());
}

#[test]
fn memory_consolidation_summary_stable() {
    let job = make_job();
    let mut j1 = ConsolidationJournal::default();
    let mut j2 = ConsolidationJournal::default();

    let o1 = job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j1,
    );
    let o2 = job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j2,
    );

    assert_eq!(o1.summary.dropped_count, o2.summary.dropped_count);
    assert_eq!(o1.summary.summary, o2.summary.summary);
}

#[test]
fn memory_consolidation_promoted_stable() {
    let job = make_job();
    let mut j1 = ConsolidationJournal::default();
    let mut j2 = ConsolidationJournal::default();

    let o1 = job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j1,
    );
    let o2 = job.run(
        &sample_turns(),
        sample_salience(),
        &sample_episodic(),
        1,
        &mut j2,
    );

    let keys1: Vec<&str> = o1.promoted.iter().map(|p| p.key.as_str()).collect();
    let keys2: Vec<&str> = o2.promoted.iter().map(|p| p.key.as_str()).collect();
    assert_eq!(keys1, keys2);
}
