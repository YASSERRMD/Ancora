use crate::forgetting::ForgettingPolicy;
use crate::salience::{SalienceItem, SalienceScorer};

fn item(key: &str, importance: u32, access_count: u32, age_secs: u64) -> SalienceItem {
    SalienceItem {
        key: key.into(),
        content: "c".into(),
        importance,
        access_count,
        age_secs,
    }
}

#[test]
fn high_age_item_forgotten() {
    let pol = ForgettingPolicy::new(0.0, 100);
    let scorer = SalienceScorer::default_weights();
    let old = item("old", 1, 0, 200);
    assert!(pol.should_forget(&old, &scorer));
}

#[test]
fn low_salience_item_forgotten() {
    let pol = ForgettingPolicy::new(100.0, u64::MAX);
    let scorer = SalienceScorer::default_weights();
    let low = item("low", 0, 0, 0);
    assert!(pol.should_forget(&low, &scorer));
}

#[test]
fn prune_drops_low_salience_old_items() {
    let pol = ForgettingPolicy::new(0.0, 50);
    let scorer = SalienceScorer::default_weights();
    let items = vec![item("keep", 10, 10, 10), item("drop", 1, 0, 100)];
    let retained = pol.prune(items, &scorer);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].key, "keep");
}

#[test]
fn prune_keeps_important_recent_items() {
    let pol = ForgettingPolicy::new(5.0, 10_000);
    let scorer = SalienceScorer::default_weights();
    let items = vec![item("important", 10, 5, 0)];
    let retained = pol.prune(items, &scorer);
    assert_eq!(retained.len(), 1);
}
