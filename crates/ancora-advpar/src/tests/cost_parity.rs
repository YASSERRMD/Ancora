use ancora_memcon::TokenBudget;

#[test]
fn cost_parity_token_estimate_formula() {
    // Formula: (len + 3) / 4 (integer division)
    assert_eq!(TokenBudget::estimate_tokens(""), 0);
    assert_eq!(TokenBudget::estimate_tokens("abcd"), 1); // 4 chars -> 1
    assert_eq!(TokenBudget::estimate_tokens("abcde"), 2); // 5 chars -> 2
    assert_eq!(TokenBudget::estimate_tokens("abcdefgh"), 2); // 8 chars -> 2
    assert_eq!(TokenBudget::estimate_tokens("abcdefghi"), 3); // 9 chars -> 3
}

#[test]
fn cost_parity_within_budget() {
    let budget = TokenBudget::new(10);
    // 40 chars / 4 = 10 tokens, fits exactly
    let content = "a".repeat(40);
    assert!(budget.within_budget(&[content.clone()]));
    // 41 chars -> 11 tokens, does not fit
    let too_long = "a".repeat(41);
    assert!(!budget.within_budget(&[too_long]));
}

#[test]
fn cost_parity_total_tokens() {
    let c1 = "aaaa".to_string(); // 1 token
    let c2 = "bbbbbbbb".to_string(); // 2 tokens
    assert_eq!(TokenBudget::total_tokens(&[c1, c2]), 3);
}

#[test]
fn cost_parity_empty_budget() {
    let budget = TokenBudget::new(0);
    assert!(!budget.within_budget(&["a".to_string()]));
    assert!(budget.within_budget(&[]));
}
