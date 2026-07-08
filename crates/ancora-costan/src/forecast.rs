//! Cost forecasting using simple linear regression and exponential smoothing.

#[derive(Debug, Clone)]
pub struct ForecastPoint {
    pub period: u32,
    pub predicted_cost: f64,
}

#[derive(Debug, Clone, Default)]
pub struct CostForecaster {
    /// Observed (period, cost) pairs.
    observations: Vec<(u32, f64)>,
}

impl CostForecaster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_observation(&mut self, period: u32, cost: f64) {
        self.observations.push((period, cost));
    }

    /// Linear regression: fit y = a + b*x over recorded observations.
    fn linear_params(&self) -> Option<(f64, f64)> {
        let n = self.observations.len();
        if n < 2 {
            return None;
        }
        let n_f = n as f64;
        let sum_x: f64 = self.observations.iter().map(|(x, _)| *x as f64).sum();
        let sum_y: f64 = self.observations.iter().map(|(_, y)| y).sum();
        let sum_xx: f64 = self
            .observations
            .iter()
            .map(|(x, _)| (*x as f64).powi(2))
            .sum();
        let sum_xy: f64 = self.observations.iter().map(|(x, y)| *x as f64 * y).sum();
        let denom = n_f * sum_xx - sum_x * sum_x;
        if denom.abs() < 1e-12 {
            return None;
        }
        let b = (n_f * sum_xy - sum_x * sum_y) / denom;
        let a = (sum_y - b * sum_x) / n_f;
        Some((a, b))
    }

    /// Forecast cost for a future period using linear regression.
    pub fn forecast_linear(&self, period: u32) -> Option<f64> {
        let (a, b) = self.linear_params()?;
        let prediction = a + b * period as f64;
        Some(prediction.max(0.0))
    }

    /// Forecast multiple periods ahead.
    pub fn forecast_next_n(&self, from_period: u32, n: u32) -> Vec<ForecastPoint> {
        (0..n)
            .filter_map(|i| {
                let p = from_period + i;
                self.forecast_linear(p).map(|cost| ForecastPoint {
                    period: p,
                    predicted_cost: cost,
                })
            })
            .collect()
    }

    /// Exponential smoothing forecast (alpha in [0,1]).
    pub fn forecast_ema(&self, alpha: f64) -> Option<f64> {
        if self.observations.is_empty() {
            return None;
        }
        let mut ema = self.observations[0].1;
        for (_, cost) in &self.observations[1..] {
            ema = alpha * cost + (1.0 - alpha) * ema;
        }
        Some(ema)
    }

    pub fn observations(&self) -> &[(u32, f64)] {
        &self.observations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_forecast_matches_trend() {
        let mut f = CostForecaster::new();
        // cost = 1.0 + 0.5 * period
        for i in 0u32..5 {
            f.add_observation(i, 1.0 + 0.5 * i as f64);
        }
        let pred = f.forecast_linear(10).unwrap();
        // expected: 1.0 + 0.5 * 10 = 6.0
        assert!((pred - 6.0).abs() < 0.01);
    }
}
