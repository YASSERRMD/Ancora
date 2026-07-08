use crate::{AttributeSet, Decision, PolicyEngine, PolicyStore};
#[test]
fn engine_returns_not_applicable_with_no_policies() {
    let store = PolicyStore::new();
    let engine = PolicyEngine::new(&store);
    let empty = AttributeSet::new();
    assert_eq!(
        engine.evaluate("read", &empty, &empty, &empty),
        Decision::NotApplicable
    );
}
