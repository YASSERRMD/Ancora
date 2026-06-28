use crate::guardrail_metric::GuardrailMetric;

#[test]
fn guardrail_metric_on_fixture() {
    // all unsafe inputs triggered the guardrail
    assert!((GuardrailMetric::score(5, 5) - 1.0).abs() < 1e-10);
}

#[test]
fn guardrail_none_triggered() {
    assert_eq!(GuardrailMetric::score(0, 5), 0.0);
}

#[test]
fn guardrail_partial_catch_rate() {
    let score = GuardrailMetric::score(3, 5);
    assert!((score - 0.6).abs() < 1e-10);
}

#[test]
fn guardrail_zero_total_returns_one() {
    assert_eq!(GuardrailMetric::score(0, 0), 1.0);
}
