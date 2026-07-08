use crate::episodic::{EpisodicEntry, EpisodicToSemanticPromoter};
use crate::forgetting::ForgettingPolicy;
use crate::job::ConsolidationJob;
use crate::journal::ConsolidationJournal;
use crate::salience::{SalienceItem, SalienceScorer};
use crate::summarizer::{ConversationSummarizer, SummarizationPolicy, Turn};

fn make_job() -> ConsolidationJob {
    ConsolidationJob {
        summarizer: ConversationSummarizer::new(SummarizationPolicy::new(3, 1)),
        scorer: SalienceScorer::default_weights(),
        promoter: EpisodicToSemanticPromoter::new(2),
        forgetting: ForgettingPolicy::new(0.0, 100_000),
    }
}

fn turns(n: usize) -> Vec<Turn> {
    (0..n)
        .map(|i| Turn {
            index: i,
            role: "user".into(),
            content: format!("t{i}"),
        })
        .collect()
}

fn sal_item(key: &str, imp: u32) -> SalienceItem {
    SalienceItem {
        key: key.into(),
        content: key.into(),
        importance: imp,
        access_count: 1,
        age_secs: 0,
    }
}

#[test]
fn job_runs_and_journals_summarize_event() {
    let job = make_job();
    let mut journal = ConsolidationJournal::default();
    let output = job.run(&turns(3), vec![], &[], 1, &mut journal);
    assert!(
        output.summary.dropped_count > 0
            || !output.summary.summary.is_empty()
            || output.summary.kept.len() == 1
    );
    assert!(!journal.entries().is_empty());
}

#[test]
fn job_promotes_recurring_episodic_facts() {
    let job = make_job();
    let mut journal = ConsolidationJournal::default();
    let episodic = vec![EpisodicEntry {
        key: "fact".into(),
        content: "recurring".into(),
        occurrences: 3,
    }];
    let out = job.run(&turns(2), vec![], &episodic, 1, &mut journal);
    assert_eq!(out.promoted.len(), 1);
}

#[test]
fn job_deduplicates_salience_items() {
    let job = make_job();
    let mut journal = ConsolidationJournal::default();
    let items = vec![sal_item("x", 5), sal_item("x", 5), sal_item("y", 3)];
    let out = job.run(&turns(1), items, &[], 1, &mut journal);
    let keys: Vec<&str> = out.retained.iter().map(|i| i.key.as_str()).collect();
    let has_dup = {
        let mut k = keys.clone();
        k.sort();
        let deduped_len = {
            let mut k2 = k.clone();
            k2.dedup();
            k2.len()
        };
        k.len() != deduped_len
    };
    assert!(!has_dup);
}
