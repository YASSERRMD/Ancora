use ancora_ageval::{MemoryMetric, PlanningMetric, ReasoningMetric};
use ancora_memcon::TokenBudget;

#[test]
fn combined_cost_bounded() {
    // Total token budget shared across pipeline steps
    let budget = TokenBudget::new(1000);
    let contents = vec![
        "short step result".to_string(),
        "another result with more detail".to_string(),
    ];
    assert!(budget.within_budget(&contents));

    // Verify all eval scores are within [0.0, 1.0]
    let scores = [
        PlanningMetric::score(&["a".into(), "b".into()], &["a".into(), "b".into()]),
        ReasoningMetric::score(4, 5),
        MemoryMetric::score(8, 10),
    ];
    for score in &scores {
        assert!(
            *score >= 0.0 && *score <= 1.0,
            "score {} out of bounds",
            score
        );
    }
}

#[test]
fn budget_exceeded_when_too_much_content() {
    let budget = TokenBudget::new(5); // very small budget
    let large = vec!["a".repeat(200)];
    assert!(!budget.within_budget(&large));
}
