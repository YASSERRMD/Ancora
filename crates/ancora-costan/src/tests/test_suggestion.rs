use crate::suggestions::{SuggestionEngine, SuggestionKind};

#[test]
fn suggestion_identifies_cheaper_model_path() {
    // Top model accounts for 80% of cost -> should suggest cheaper model
    let model_costs = vec![("claude-opus".to_string(), 800.0), ("claude-haiku".to_string(), 200.0)];
    let tool_costs: Vec<(String, f64)> = vec![];
    let suggestions = SuggestionEngine::analyze(&model_costs, 0.9, &tool_costs, 1000.0);
    let has_cheaper = suggestions.iter().any(|s| s.kind == SuggestionKind::UseCheeperModel);
    assert!(has_cheaper, "should suggest a cheaper model when one model dominates cost");
}

#[test]
fn low_cache_rate_triggers_cache_suggestion() {
    let model_costs = vec![("gpt-4".to_string(), 50.0)];
    let tool_costs: Vec<(String, f64)> = vec![];
    let suggestions = SuggestionEngine::analyze(&model_costs, 0.05, &tool_costs, 100.0);
    let has_cache = suggestions.iter().any(|s| s.kind == SuggestionKind::EnableCaching);
    assert!(has_cache, "cache hit rate of 5% should trigger caching suggestion");
}

#[test]
fn expensive_tool_triggers_dedup_suggestion() {
    let model_costs = vec![("claude-sonnet".to_string(), 50.0)];
    let tool_costs = vec![("search".to_string(), 30.0)];
    let suggestions = SuggestionEngine::analyze(&model_costs, 0.9, &tool_costs, 100.0);
    let has_tool = suggestions
        .iter()
        .any(|s| s.kind == SuggestionKind::DeduplicateTools);
    assert!(has_tool, "expensive tool should trigger dedup suggestion");
}

#[test]
fn suggestions_sorted_by_savings_descending() {
    let model_costs = vec![("claude-opus".to_string(), 800.0)];
    let tool_costs = vec![("search".to_string(), 300.0)];
    let suggestions = SuggestionEngine::analyze(&model_costs, 0.1, &tool_costs, 1000.0);
    for i in 1..suggestions.len() {
        assert!(
            suggestions[i - 1].estimated_savings_usd >= suggestions[i].estimated_savings_usd,
            "suggestions should be sorted by savings desc"
        );
    }
}
