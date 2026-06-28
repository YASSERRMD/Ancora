use crate::presets::allow_if_classification_at_most;
use crate::{AttributeSet, Decision, PolicyEngine, PolicyStore};
#[test]
fn classification_allows_below_max() {
    let mut store = PolicyStore::new();
    store.add(allow_if_classification_at_most(2));
    let mut r = AttributeSet::new(); r.set("classification_level", 1i64);
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("read", &e, &r, &e), Decision::Allow);
}
#[test]
fn classification_denies_above_max() {
    let mut store = PolicyStore::new();
    store.add(allow_if_classification_at_most(2));
    let mut r = AttributeSet::new(); r.set("classification_level", 4i64);
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("read", &e, &r, &e), Decision::NotApplicable);
}
