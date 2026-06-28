//! Routing cost-quality metric: harmonic-style blend of quality and cost efficiency.

pub struct RoutingMetric;

impl RoutingMetric {
    pub const NAME: &'static str = "routing_cost_quality";

    /// Score = (quality + cost_efficiency) / 2.
    /// `quality` in [0.0, 1.0]. `cost` is normalized against `max_cost`.
    /// Returns `quality` if `max_cost` is zero.
    pub fn score(quality: f64, cost: u64, max_cost: u64) -> f64 {
        if max_cost == 0 {
            return quality;
        }
        let cost_efficiency = 1.0 - (cost as f64 / max_cost as f64);
        (quality + cost_efficiency) / 2.0
    }
}
