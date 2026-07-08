use ancora_memcon::TokenBudget;

fn content_set() -> Vec<String> {
    vec![
        "The agent framework provides structured reasoning capabilities.".into(),
        "Memory consolidation reduces token footprint across turns.".into(),
        "Guardrails block injection and PII at ingress.".into(),
    ]
}

#[test]
fn cost_token_estimate_stable() {
    let contents = content_set();
    let t1 = TokenBudget::total_tokens(&contents);
    let t2 = TokenBudget::total_tokens(&contents);
    assert_eq!(t1, t2);
}

#[test]
fn cost_budget_within_stable() {
    let budget = TokenBudget::new(1000);
    let contents = content_set();
    let r1 = budget.within_budget(&contents);
    let r2 = budget.within_budget(&contents);
    assert_eq!(r1, r2);
}

#[test]
fn cost_single_estimate_stable() {
    let s = "hello world this is a test string";
    let e1 = TokenBudget::estimate_tokens(s);
    let e2 = TokenBudget::estimate_tokens(s);
    assert_eq!(e1, e2);
    // roughly 1 token per 4 chars
    assert_eq!(e1, s.len().div_ceil(4));
}
