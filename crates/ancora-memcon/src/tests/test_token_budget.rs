use crate::token_budget::TokenBudget;

#[test]
fn estimate_tokens_is_roughly_chars_over_4() {
    assert_eq!(TokenBudget::estimate_tokens("abcd"), 1);
    assert_eq!(TokenBudget::estimate_tokens("abcde"), 2);
}

#[test]
fn total_tokens_sums_all_contents() {
    let contents = vec!["abcd".to_string(), "efgh".to_string()];
    assert_eq!(TokenBudget::total_tokens(&contents), 2);
}

#[test]
fn within_budget_passes_when_small() {
    let budget = TokenBudget::new(100);
    let contents = vec!["hello".to_string()];
    assert!(budget.within_budget(&contents));
}

#[test]
fn within_budget_fails_when_large() {
    let budget = TokenBudget::new(1);
    let contents = vec!["a very long string that exceeds the budget limit".to_string()];
    assert!(!budget.within_budget(&contents));
}

#[test]
fn token_footprint_reduced_after_consolidation() {
    let before = vec![
        "turn1".to_string(),
        "turn2".to_string(),
        "turn3".to_string(),
        "turn4".to_string(),
        "turn5".to_string(),
    ];
    let after = vec!["summary of 4 turns: [...]".to_string(), "turn5".to_string()];
    let before_tokens = TokenBudget::total_tokens(&before);
    let after_tokens = TokenBudget::total_tokens(&after);
    assert!(after_tokens <= before_tokens + 5);
}
