use ancora_abac::*;

fn main() {
    let mut store = PolicyStore::new();
    store.add(Policy::new(
        "allow-eng-read",
        Effect::Allow,
        vec!["read".into()],
        Condition::Eq("dept".into(), AttributeValue::String("engineering".into())),
    ).with_priority(10));
    store.add(Policy::new(
        "deny-blocked",
        Effect::Deny,
        vec!["*".into()],
        Condition::Eq("blocked".into(), AttributeValue::Bool(true)),
    ).with_priority(5));

    let engine = PolicyEngine::new(&store);

    let alice = RequestContext::new().with_subject_attr("dept", "engineering");
    let bob = RequestContext::new().with_subject_attr("dept", "engineering").with_subject_attr("blocked", true);
    let carol = RequestContext::new().with_subject_attr("dept", "hr");

    for (name, ctx) in [("alice", alice), ("bob", bob), ("carol", carol)] {
        let d = engine.evaluate("read", &ctx.subject, &ctx.resource, &ctx.environment);
        println!("{name}: {:?}", d);
    }
}
