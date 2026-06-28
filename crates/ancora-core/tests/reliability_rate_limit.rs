// Reliability: token-bucket rate limiter for outbound API calls.

struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_per_tick: f64,
}

impl TokenBucket {
    fn new(capacity: f64, refill_per_tick: f64) -> Self {
        Self { tokens: capacity, capacity, refill_per_tick }
    }

    fn tick(&mut self) {
        self.tokens = (self.tokens + self.refill_per_tick).min(self.capacity);
    }

    fn try_consume(&mut self, cost: f64) -> bool {
        if self.tokens >= cost {
            self.tokens -= cost;
            true
        } else {
            false
        }
    }
}

#[test]
fn test_consume_within_capacity_succeeds() {
    let mut b = TokenBucket::new(10.0, 1.0);
    assert!(b.try_consume(5.0));
    assert!(b.try_consume(5.0));
}

#[test]
fn test_consume_beyond_capacity_fails() {
    let mut b = TokenBucket::new(10.0, 1.0);
    assert!(!b.try_consume(11.0));
}

#[test]
fn test_tokens_refill_on_tick() {
    let mut b = TokenBucket::new(10.0, 3.0);
    b.try_consume(10.0);
    b.tick();
    assert!((b.tokens - 3.0).abs() < 0.001);
}

#[test]
fn test_tokens_do_not_exceed_capacity_on_refill() {
    let mut b = TokenBucket::new(5.0, 10.0);
    b.tick();
    assert!((b.tokens - 5.0).abs() < 0.001);
}

#[test]
fn test_burst_then_rate_limited() {
    let mut b = TokenBucket::new(5.0, 1.0);
    assert!(b.try_consume(5.0));
    assert!(!b.try_consume(1.0));
    b.tick();
    assert!(b.try_consume(1.0));
}

#[test]
fn test_partial_consume_updates_tokens() {
    let mut b = TokenBucket::new(10.0, 0.0);
    b.try_consume(3.0);
    assert!((b.tokens - 7.0).abs() < 0.001);
}
