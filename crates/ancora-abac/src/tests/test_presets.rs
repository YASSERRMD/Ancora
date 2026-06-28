use crate::presets::{allow_if_department, deny_if_blocked};
use crate::{AttributeSet, Decision, PolicyEngine, PolicyStore};
#[test]
fn dept_preset_allows_correct_dept() {
    let mut store = PolicyStore::new();
    store.add(allow_if_department("eng"));
    let mut s = AttributeSet::new(); s.set("department", "eng");
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("anything", &s, &e, &e), Decision::Allow);
}
#[test]
fn deny_blocked_preset_fires() {
    let mut store = PolicyStore::new();
    store.add(deny_if_blocked());
    let mut s = AttributeSet::new(); s.set("blocked", true);
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert!(matches!(engine.evaluate("read", &s, &e, &e), Decision::Deny(_)));
}
#[test]
fn dept_preset_denies_wrong_dept() {
    let mut store = PolicyStore::new();
    store.add(allow_if_department("eng"));
    let mut s = AttributeSet::new(); s.set("department", "hr");
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("anything", &s, &e, &e), Decision::NotApplicable);
}
