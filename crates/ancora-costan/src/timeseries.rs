/// Cost time series tracking - records cost data points over time.

#[derive(Debug, Clone, PartialEq)]
pub struct CostPoint {
    /// Unix timestamp in seconds.
    pub timestamp: u64,
    /// Cost in USD (fractional).
    pub cost_usd: f64,
    /// Number of tokens consumed at this point.
    pub tokens: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CostTimeSeries {
    points: Vec<CostPoint>,
}

impl CostTimeSeries {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Record a new cost data point.
    pub fn record(&mut self, timestamp: u64, cost_usd: f64, tokens: u64) {
        self.points.push(CostPoint {
            timestamp,
            cost_usd,
            tokens,
        });
        self.points.sort_by_key(|p| p.timestamp);
    }

    /// Return all recorded points.
    pub fn points(&self) -> &[CostPoint] {
        &self.points
    }

    /// Sum total cost over all recorded points.
    pub fn total_cost(&self) -> f64 {
        self.points.iter().map(|p| p.cost_usd).sum()
    }

    /// Sum total tokens over all recorded points.
    pub fn total_tokens(&self) -> u64 {
        self.points.iter().map(|p| p.tokens).sum()
    }

    /// Return points within the given time range [start, end] inclusive.
    pub fn range(&self, start: u64, end: u64) -> Vec<&CostPoint> {
        self.points
            .iter()
            .filter(|p| p.timestamp >= start && p.timestamp <= end)
            .collect()
    }

    /// Rolling window average cost over the last `window` points.
    pub fn rolling_avg(&self, window: usize) -> Option<f64> {
        if self.points.is_empty() || window == 0 {
            return None;
        }
        let len = self.points.len();
        let slice = &self.points[len.saturating_sub(window)..];
        let sum: f64 = slice.iter().map(|p| p.cost_usd).sum();
        Some(sum / slice.len() as f64)
    }

    /// Aggregate by hour bucket (truncate timestamp to hour).
    pub fn hourly_buckets(&self) -> Vec<(u64, f64)> {
        let mut buckets: std::collections::BTreeMap<u64, f64> = std::collections::BTreeMap::new();
        for p in &self.points {
            let hour = (p.timestamp / 3600) * 3600;
            *buckets.entry(hour).or_insert(0.0) += p.cost_usd;
        }
        buckets.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_cost_sums_correctly() {
        let mut ts = CostTimeSeries::new();
        ts.record(1000, 0.50, 100);
        ts.record(2000, 0.25, 50);
        let total = ts.total_cost();
        assert!((total - 0.75).abs() < 1e-9);
    }

    #[test]
    fn range_filters_correctly() {
        let mut ts = CostTimeSeries::new();
        ts.record(1000, 0.10, 10);
        ts.record(2000, 0.20, 20);
        ts.record(3000, 0.30, 30);
        let in_range = ts.range(1500, 2500);
        assert_eq!(in_range.len(), 1);
        assert_eq!(in_range[0].timestamp, 2000);
    }
}
