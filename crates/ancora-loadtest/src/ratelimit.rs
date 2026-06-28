/// Token bucket rate limiter for controlling test request dispatch rate.
pub struct TokenBucket {
    pub max_tokens: u64,
    pub refill_per_sec: f64,
    tokens: f64,
    last_refill_secs: u64,
}

impl TokenBucket {
    pub fn new(max_tokens: u64, refill_per_sec: f64) -> Self {
        Self {
            max_tokens,
            refill_per_sec,
            tokens: max_tokens as f64,
            last_refill_secs: 0,
        }
    }

    pub fn refill(&mut self, now: u64) {
        let elapsed = now.saturating_sub(self.last_refill_secs) as f64;
        self.tokens = (self.tokens + elapsed * self.refill_per_sec).min(self.max_tokens as f64);
        self.last_refill_secs = now;
    }

    pub fn try_consume(&mut self, tokens: u64) -> bool {
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }

    pub fn available(&self) -> u64 {
        self.tokens as u64
    }
}
