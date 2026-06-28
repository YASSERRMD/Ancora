use ancora_ageval::{CoordinationMetric, GuardrailMetric, MemoryMetric, ReasoningMetric};

const EPS: f64 = 1e-9;

#[test]
fn coordination_perfect_score() {
    assert!((CoordinationMetric::score(3, 3) - 1.0).abs() < EPS);
}

#[test]
fn guardrail_half_score() {
    assert!((GuardrailMetric::score(1, 2) - 0.5).abs() < EPS);
}

#[test]
fn reasoning_canonical_score() {
    assert!((ReasoningMetric::score(4, 5) - 0.8).abs() < EPS);
}

#[test]
fn memory_canonical_score() {
    assert!((MemoryMetric::score(9, 10) - 0.9).abs() < EPS);
}
