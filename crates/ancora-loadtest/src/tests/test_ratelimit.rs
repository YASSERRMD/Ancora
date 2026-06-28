use crate::ratelimit::TokenBucket;

#[test]
fn consume_within_budget_succeeds() {
    let mut tb = TokenBucket::new(10, 1.0);
    assert!(tb.try_consume(5));
    assert_eq!(tb.available(), 5);
}

#[test]
fn consume_over_budget_fails() {
    let mut tb = TokenBucket::new(5, 1.0);
    assert!(!tb.try_consume(6));
}

#[test]
fn refill_increases_tokens() {
    let mut tb = TokenBucket::new(10, 5.0);
    tb.try_consume(10);
    tb.refill(2); // 2 seconds * 5/sec = 10 tokens
    assert_eq!(tb.available(), 10);
}

#[test]
fn tokens_capped_at_max() {
    let mut tb = TokenBucket::new(10, 100.0);
    tb.refill(10); // would be 1000 tokens but capped at 10
    assert_eq!(tb.available(), 10);
}
