/// Cache-hit savings tracking - measures cost avoided through cache hits.

#[derive(Debug, Clone, Default)]
pub struct CacheSavingsTracker {
    /// Total cost that would have been incurred without caching.
    full_cost: f64,
    /// Cost actually paid (cache misses only).
    actual_cost: f64,
    /// Number of cache hits.
    hits: u64,
    /// Number of cache misses.
    misses: u64,
    /// Tokens served from cache.
    cached_tokens: u64,
    /// Tokens computed fresh.
    computed_tokens: u64,
}

impl CacheSavingsTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a cache hit: cost_if_computed is what it would have cost; actual is what was paid.
    pub fn record_hit(&mut self, cost_if_computed: f64, actual_cost: f64, tokens: u64) {
        self.full_cost += cost_if_computed;
        self.actual_cost += actual_cost;
        self.hits += 1;
        self.cached_tokens += tokens;
    }

    /// Record a cache miss: full cost paid.
    pub fn record_miss(&mut self, cost_usd: f64, tokens: u64) {
        self.full_cost += cost_usd;
        self.actual_cost += cost_usd;
        self.misses += 1;
        self.computed_tokens += tokens;
    }

    /// Total savings in USD.
    pub fn total_savings(&self) -> f64 {
        (self.full_cost - self.actual_cost).max(0.0)
    }

    /// Hit rate as a fraction in [0.0, 1.0].
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// Total full cost (without cache).
    pub fn full_cost(&self) -> f64 {
        self.full_cost
    }

    /// Total actual cost paid.
    pub fn actual_cost(&self) -> f64 {
        self.actual_cost
    }

    /// Number of cache hits.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Number of cache misses.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Tokens served from cache.
    pub fn cached_tokens(&self) -> u64 {
        self.cached_tokens
    }

    /// Saving percentage (0-100).
    pub fn saving_percentage(&self) -> f64 {
        if self.full_cost == 0.0 {
            return 0.0;
        }
        (self.total_savings() / self.full_cost) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savings_computed_correctly() {
        let mut t = CacheSavingsTracker::new();
        t.record_hit(1.0, 0.1, 500);
        t.record_miss(0.5, 250);
        assert!((t.total_savings() - 0.9).abs() < 1e-9);
        assert!((t.hit_rate() - 0.5).abs() < 1e-9);
    }
}
