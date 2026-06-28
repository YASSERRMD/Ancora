use crate::baseline::{Baseline, BaselineStore};
use std::collections::HashMap;

#[test]
fn baseline_updates_on_approved_change() {
    let mut store = BaselineStore::new();

    let mut baseline = Baseline::new("mmlu");
    baseline.set("accuracy", 0.85);
    store.upsert(baseline);

    // Simulate approved improvement: update baseline with new values.
    let mut new_values = HashMap::new();
    new_values.insert("accuracy".to_string(), 0.88);

    if let Some(b) = store.get_mut("mmlu") {
        b.update_from(&new_values);
    }

    let updated = store.get("mmlu").unwrap();
    assert!(
        (updated.get("accuracy").unwrap() - 0.88).abs() < 1e-9,
        "baseline should be updated to new approved value"
    );
}

#[test]
fn baseline_preserves_other_metrics_on_update() {
    let mut baseline = Baseline::new("hellaswag");
    baseline.set("accuracy", 0.70);
    baseline.set("f1", 0.68);

    let mut updates = HashMap::new();
    updates.insert("accuracy".to_string(), 0.75);
    baseline.update_from(&updates);

    assert!((baseline.get("accuracy").unwrap() - 0.75).abs() < 1e-9);
    assert!((baseline.get("f1").unwrap() - 0.68).abs() < 1e-9);
}
