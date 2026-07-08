//! Example: compose orchestration, guardrails, reasoning, and eval into one pipeline.

use ancora_ageval::{EvalReport, GuardrailMetric, MetricScore, ReasoningMetric};
use ancora_guard::{
    GuardrailJournal, GuardrailPolicy, InjectionInputGuardrail, SafetyOutputGuardrail,
};
use ancora_orchestrate::fan_out;
use ancora_reason::{ReasoningEvent, ReasoningJournal, StepDecomposer, StepVerifier};

pub fn run() {
    // Step 1: fan out tasks
    let inputs = vec![serde_json::json!("claim-A"), serde_json::json!("claim-B")];
    let tasks = fan_out("pipeline-orch", "worker", inputs, "root");
    println!("Spawned {} tasks", tasks.len());

    // Step 2: guard each task input
    let mut guard_policy = GuardrailPolicy::new();
    guard_policy.add_input(InjectionInputGuardrail);
    guard_policy.add_output(SafetyOutputGuardrail);
    let mut guard_journal = GuardrailJournal::default();
    let mut blocked = 0usize;
    let total = tasks.len();
    for task in &tasks {
        let input = task.input.as_str().unwrap_or("input");
        let outcome = guard_policy.check_input(input, &mut guard_journal, 1);
        if matches!(outcome, ancora_guard::GuardrailOutcome::Block(_)) {
            blocked += 1;
        }
    }

    // Step 3: reason about claims
    let claims: Vec<String> = tasks
        .iter()
        .map(|t| t.input.as_str().unwrap_or("claim").to_string())
        .collect();
    let mut steps = StepDecomposer::decompose(claims);
    let mut reason_journal = ReasoningJournal::default();
    let mut verified = 0usize;
    for step in steps.iter_mut() {
        reason_journal.record(
            step.index as u64,
            ReasoningEvent::StepAdded {
                index: step.index,
                claim: step.claim.clone(),
            },
        );
        let result = StepVerifier::verify(step, |c| !c.is_empty());
        if result.passed {
            verified += 1;
            reason_journal.record(
                step.index as u64,
                ReasoningEvent::StepVerified { index: step.index },
            );
        }
    }

    // Step 4: eval report
    let mut report = EvalReport::new("advanced-pipeline", 1);
    report.add_score(MetricScore::new(
        "guardrail_catch_rate",
        GuardrailMetric::score(blocked, total),
    ));
    report.add_score(MetricScore::new(
        "reasoning_correctness",
        ReasoningMetric::score(verified, steps.len()),
    ));
    println!("{}", report.summary());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advanced_combined_example_runs() {
        run();
    }
}
