use std::time::Instant;

use ancora_orchestrate::fan_out;
use ancora_reason::StepDecomposer;
use ancora_guard::{GuardrailJournal, GuardrailPolicy, SafetyOutputGuardrail};

#[test]
fn combined_pipeline_overhead() {
    let start = Instant::now();

    // Fan out 100 tasks
    let inputs: Vec<serde_json::Value> = (0..100).map(|i| serde_json::json!(format!("input-{}", i))).collect();
    let tasks = fan_out("bench-orch", "agent", inputs, "root");
    assert_eq!(tasks.len(), 100);

    // Decompose 50 reasoning steps
    let claims: Vec<String> = (0..50).map(|i| format!("claim-{}", i)).collect();
    let steps = StepDecomposer::decompose(claims);
    assert_eq!(steps.len(), 50);

    // Run 200 guardrail checks
    let mut policy = GuardrailPolicy::new();
    policy.add_output(SafetyOutputGuardrail);
    let mut journal = GuardrailJournal::default();
    for i in 0_u64..200 {
        policy.check_output(&format!("safe output {}", i), &mut journal, i);
    }

    let elapsed_ms = start.elapsed().as_millis();
    assert!(elapsed_ms < 500, "combined pipeline took {}ms (limit 500ms)", elapsed_ms);
}
