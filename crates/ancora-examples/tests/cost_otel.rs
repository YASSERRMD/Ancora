use ancora_examples::{Span, TokenEstimator};

#[test]
fn span_records_name_and_duration() {
    let mut s = Span::new("agent.run");
    s.set_attribute("run.id", "abc");
    let ms = s.end_ms();
    assert!(ms < 1_000, "should complete in under a second");
    assert_eq!(Some(ms), s.duration_ms);
    assert_eq!("abc", s.attributes["run.id"]);
}

#[test]
fn token_estimator_returns_at_least_one() {
    assert_eq!(1, TokenEstimator::estimate_tokens(""));
    assert_eq!(1, TokenEstimator::estimate_tokens("abcd"));
}

#[test]
fn token_estimator_four_chars_per_token() {
    assert_eq!(2, TokenEstimator::estimate_tokens("abcde"));
    assert_eq!(25, TokenEstimator::estimate_tokens(&"x".repeat(100)));
}

#[test]
fn span_attributes_are_readable_after_end() {
    let mut s = Span::new("agent.summary");
    s.set_attribute("events", "5");
    s.set_attribute("tokens.estimated", "136");
    s.end_ms();

    assert_eq!("5", s.attributes["events"]);
    assert_eq!("136", s.attributes["tokens.estimated"]);
}

#[test]
fn multiple_spans_accumulate_independently() {
    let mut root    = Span::new("root");
    let mut child_a = Span::new("child-a");
    let mut child_b = Span::new("child-b");

    child_a.set_attribute("tokens", "10");
    child_b.set_attribute("tokens", "20");
    let total: usize = ["10", "20"]
        .iter()
        .map(|v| v.parse::<usize>().unwrap())
        .sum();
    root.set_attribute("tokens.total", total.to_string());

    child_a.end_ms();
    child_b.end_ms();
    root.end_ms();

    assert_eq!("30", root.attributes["tokens.total"]);
}

#[test]
fn cost_estimate_scales_with_text_length() {
    let short = TokenEstimator::estimate_tokens("hello");
    let long  = TokenEstimator::estimate_tokens("hello world, how are you doing today?");
    assert!(long > short);
}
