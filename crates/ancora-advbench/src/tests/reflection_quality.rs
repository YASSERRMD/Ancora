use ancora_ageval::ReflectionMetric;

#[test]
fn reflection_quality_gain_measured_grew() {
    let score = ReflectionMetric::score("short", "much longer answer with more detail");
    assert_eq!(score, 1.0, "grew response should score 1.0");
}

#[test]
fn reflection_quality_gain_measured_shrunk() {
    let score = ReflectionMetric::score("this is a long answer with many words", "short");
    assert_eq!(score, 0.5, "shrunk but changed should score 0.5");
}

#[test]
fn reflection_no_gain_unchanged() {
    let score = ReflectionMetric::score("same text", "same text");
    assert_eq!(score, 0.0, "unchanged response should score 0.0");
}

#[test]
fn reflection_quality_is_between_zero_and_one() {
    for (before, after) in [("x", "y"), ("longer string", "s"), ("a", "a"), ("", "abc")] {
        let s = ReflectionMetric::score(before, after);
        assert!(
            (0.0..=1.0).contains(&s),
            "score {s} out of [0, 1] for ({before:?}, {after:?})"
        );
    }
}
