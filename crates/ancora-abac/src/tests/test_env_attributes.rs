use crate::{AttributeSet, AttributeValue, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn env_attribute_used_in_condition() {
    let mut store = PolicyStore::new();
    store.add(Policy::new("business-hours", Effect::Allow, vec!["read".into()],
        Condition::Eq("hours".into(), AttributeValue::String("business".into()))));
    let s = AttributeSet::new();
    let mut env = AttributeSet::new(); env.set("hours", "business");
    let engine = PolicyEngine::new(&store);
    assert_eq!(engine.evaluate("read", &s, &AttributeSet::new(), &env), Decision::Allow);
    let off_env = AttributeSet::new();
    assert_eq!(engine.evaluate("read", &s, &AttributeSet::new(), &off_env), Decision::NotApplicable);
}
