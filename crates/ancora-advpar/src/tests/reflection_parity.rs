use ancora_ageval::ReflectionMetric;

const EPS: f64 = 1e-9;

#[test]
fn reflection_parity_grew() {
    assert!((ReflectionMetric::score("short", "longer answer here") - 1.0).abs() < EPS);
}

#[test]
fn reflection_parity_shrunk() {
    assert!((ReflectionMetric::score("a longer text here", "short") - 0.5).abs() < EPS);
}

#[test]
fn reflection_parity_unchanged() {
    assert!((ReflectionMetric::score("same", "same") - 0.0).abs() < EPS);
}

#[test]
fn reflection_parity_empty_both() {
    assert_eq!(ReflectionMetric::score("", ""), 0.0);
}

#[test]
fn reflection_parity_canonical_values() {
    // These exact values must match the Go and other language ports
    assert!((ReflectionMetric::score("short", "longer answer") - 1.0).abs() < EPS);
    assert!((ReflectionMetric::score("longer text here", "short") - 0.5).abs() < EPS);
    assert!((ReflectionMetric::score("x", "x") - 0.0).abs() < EPS);
}
