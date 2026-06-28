//! Example: behavior eval harness combining all metrics with regression detection.

use ancora_ageval::{
    BaselineStore, CoordinationMetric, EvalDataset, EvalReport, EvalSample, GuardrailMetric,
    MemoryMetric, MetricScore, PlanningMetric, ReasoningMetric, ReflectionMetric, RoutingMetric,
};

pub fn run() {
    let mut dataset = EvalDataset::new("advanced-capabilities-v1");
    dataset.add(EvalSample::new("plan-001").with_tag("planning").with_meta("goal", "search"));
    dataset.add(EvalSample::new("refl-001").with_tag("reflection").with_meta("goal", "critique"));
    dataset.add(EvalSample::new("route-001").with_tag("routing"));
    println!("Dataset: {} samples", dataset.len());

    let planning_score = PlanningMetric::score(
        &["search".into(), "rank".into(), "summarize".into()],
        &["search".into(), "rank".into(), "summarize".into()],
    );
    let reflection_score = ReflectionMetric::score(
        "initial answer",
        "refined answer with additional context and corrections",
    );
    let routing_score = RoutingMetric::score(0.9, 20, 100);
    let coordination_score = CoordinationMetric::score(4, 4);
    let guardrail_score = GuardrailMetric::score(3, 3);
    let reasoning_score = ReasoningMetric::score(5, 6);
    let memory_score = MemoryMetric::score(8, 10);

    let mut store = BaselineStore::new(0.05);
    store.set("planning_quality", 0.85);
    store.set("reflection_improvement", 0.9);
    store.set("routing_cost_quality", 0.8);
    store.set("coordination_success", 0.95);
    store.set("guardrail_catch_rate", 0.9);
    store.set("reasoning_correctness", 0.8);
    store.set("memory_retention", 0.85);

    let metrics = [
        ("planning_quality", planning_score),
        ("reflection_improvement", reflection_score),
        ("routing_cost_quality", routing_score),
        ("coordination_success", coordination_score),
        ("guardrail_catch_rate", guardrail_score),
        ("reasoning_correctness", reasoning_score),
        ("memory_retention", memory_score),
    ];

    let mut report = EvalReport::new("advanced-capabilities", 42);
    for (name, score) in &metrics {
        report.add_score(MetricScore::new(*name, *score));
        let result = store.check(name, *score);
        if matches!(result, ancora_ageval::BaselineResult::Regressed { .. }) {
            report.add_regression(*name);
        }
    }

    println!("{}", report.summary());
    if report.has_regressions() {
        println!("Regressions: {:?}", report.regressions);
    } else {
        println!("No regressions detected.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_eval_harness_runs() {
        run();
    }
}
