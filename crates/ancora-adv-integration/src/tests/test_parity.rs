// Cross-language parity: document the expected API surface for non-Rust SDKs.
// Each assertion verifies that the Rust implementation produces the canonical
// result that all language SDKs should match.

use ancora_ageval::{
    CoordinationMetric, GuardrailMetric, MemoryMetric, PlanningMetric, ReasoningMetric,
    ReflectionMetric, RoutingMetric,
};

#[test]
fn cross_language_parity_for_advanced_features() {
    // These canonical values serve as the reference for all SDK ports.
    // planning: 2/3 steps matched
    assert!(
        (PlanningMetric::score(
            &["a".into(), "b".into(), "c".into()],
            &["a".into(), "b".into()]
        ) - 2.0 / 3.0)
            .abs()
            < 1e-9
    );

    // reflection: changed and longer = 1.0
    assert_eq!(
        ReflectionMetric::score("short", "much longer improved answer"),
        1.0
    );

    // routing: quality=1.0, cost=0 -> score=1.0
    assert_eq!(RoutingMetric::score(1.0, 0, 100), 1.0);

    // coordination: 5/5 = 1.0
    assert_eq!(CoordinationMetric::score(5, 5), 1.0);

    // guardrail: 3/4 = 0.75
    assert!((GuardrailMetric::score(3, 4) - 0.75).abs() < 1e-9);

    // reasoning: 4/5 = 0.8
    assert!((ReasoningMetric::score(4, 5) - 0.8).abs() < 1e-9);

    // memory: 9/10 = 0.9
    assert!((MemoryMetric::score(9, 10) - 0.9).abs() < 1e-9);
}
