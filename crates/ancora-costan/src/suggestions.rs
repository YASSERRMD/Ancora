/// Cost optimization suggestions engine.

#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionKind {
    /// Switch to a cheaper model for a given use case.
    UseCheeperModel,
    /// Enable or expand caching to reduce redundant calls.
    EnableCaching,
    /// Batch requests to reduce per-request overhead.
    BatchRequests,
    /// Route low-complexity tasks to smaller models.
    RouteToSmallerModel,
    /// Reduce token usage through prompt compression.
    CompressPrompts,
    /// Avoid redundant tool calls.
    DeduplicateTools,
    /// Custom suggestion.
    Custom(String),
}

impl SuggestionKind {
    pub fn description(&self) -> &str {
        match self {
            SuggestionKind::UseCheeperModel => "Switch to a cheaper model for this workload",
            SuggestionKind::EnableCaching => "Enable prompt caching to avoid re-computing common prefixes",
            SuggestionKind::BatchRequests => "Batch multiple small requests into a single call",
            SuggestionKind::RouteToSmallerModel => "Route low-complexity queries to a smaller, cheaper model",
            SuggestionKind::CompressPrompts => "Compress system prompts to reduce input token costs",
            SuggestionKind::DeduplicateTools => "Deduplicate redundant tool invocations within a single turn",
            SuggestionKind::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub kind: SuggestionKind,
    /// Estimated monthly savings in USD.
    pub estimated_savings_usd: f64,
    /// Confidence level in [0.0, 1.0].
    pub confidence: f64,
    pub detail: String,
}

impl Suggestion {
    pub fn new(
        kind: SuggestionKind,
        estimated_savings_usd: f64,
        confidence: f64,
        detail: impl Into<String>,
    ) -> Self {
        Self { kind, estimated_savings_usd, confidence, detail: detail.into() }
    }
}

pub struct SuggestionEngine;

impl SuggestionEngine {
    /// Analyze cost data and return ordered suggestions (highest savings first).
    pub fn analyze(
        model_costs: &[(String, f64)],
        cache_hit_rate: f64,
        tool_costs: &[(String, f64)],
        total_monthly_cost: f64,
    ) -> Vec<Suggestion> {
        let mut suggestions: Vec<Suggestion> = Vec::new();

        // Suggest caching if hit rate is low.
        if cache_hit_rate < 0.3 {
            let saving = total_monthly_cost * 0.25;
            suggestions.push(Suggestion::new(
                SuggestionKind::EnableCaching,
                saving,
                0.8,
                format!(
                    "Cache hit rate is {:.1}%. Enabling caching could save ~25% of total cost.",
                    cache_hit_rate * 100.0
                ),
            ));
        }

        // Suggest cheaper model if top model is expensive.
        if let Some((model, cost)) = model_costs.first() {
            if *cost > total_monthly_cost * 0.5 {
                let saving = cost * 0.40;
                suggestions.push(Suggestion::new(
                    SuggestionKind::UseCheeperModel,
                    saving,
                    0.7,
                    format!(
                        "Model '{}' accounts for {:.0}% of cost. Consider a cheaper variant for non-critical paths.",
                        model,
                        (cost / total_monthly_cost) * 100.0
                    ),
                ));
            }
        }

        // Suggest routing if there are expensive tools.
        if let Some((tool, cost)) = tool_costs.first() {
            if *cost > total_monthly_cost * 0.2 {
                let saving = cost * 0.30;
                suggestions.push(Suggestion::new(
                    SuggestionKind::DeduplicateTools,
                    saving,
                    0.6,
                    format!(
                        "Tool '{}' costs ${:.2}/month. Review for redundant invocations.",
                        tool, cost
                    ),
                ));
            }
        }

        // Suggest prompt compression if cost is high.
        if total_monthly_cost > 100.0 {
            suggestions.push(Suggestion::new(
                SuggestionKind::CompressPrompts,
                total_monthly_cost * 0.10,
                0.5,
                "Compressing system prompts could reduce input token costs by ~10%.".to_string(),
            ));
        }

        // Sort by estimated savings descending.
        suggestions.sort_by(|a, b| {
            b.estimated_savings_usd
                .partial_cmp(&a.estimated_savings_usd)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestions_returned_for_low_cache_rate() {
        let model_costs = vec![("claude-opus".to_string(), 60.0)];
        let tool_costs = vec![("search".to_string(), 5.0)];
        let suggestions =
            SuggestionEngine::analyze(&model_costs, 0.1, &tool_costs, 100.0);
        assert!(!suggestions.is_empty());
        let has_cache = suggestions
            .iter()
            .any(|s| s.kind == SuggestionKind::EnableCaching);
        assert!(has_cache, "should suggest enabling caching");
    }
}
