use crate::salience::{SalienceItem, SalienceScorer};

fn item(importance: u32, access_count: u32, age_secs: u64) -> SalienceItem {
    SalienceItem {
        key: "k".into(),
        content: "c".into(),
        importance,
        access_count,
        age_secs,
    }
}

#[test]
fn high_importance_scores_higher() {
    let scorer = SalienceScorer::default_weights();
    let high = item(10, 1, 0);
    let low = item(1, 1, 0);
    assert!(scorer.score(&high) > scorer.score(&low));
}

#[test]
fn recent_scores_higher_than_old() {
    let scorer = SalienceScorer::default_weights();
    let fresh = item(5, 1, 0);
    let old = item(5, 1, 100_000);
    assert!(scorer.score(&fresh) > scorer.score(&old));
}

#[test]
fn frequent_access_increases_score() {
    let scorer = SalienceScorer::default_weights();
    let frequent = item(5, 100, 0);
    let rare = item(5, 1, 0);
    assert!(scorer.score(&frequent) > scorer.score(&rare));
}
