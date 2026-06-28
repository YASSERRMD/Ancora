/// Forecast future cost from recent usage via linear extrapolation.
pub struct CostForecaster {
    /// Daily cost samples (most recent last).
    samples: Vec<f64>,
}

impl CostForecaster {
    pub fn new(samples: Vec<f64>) -> Self {
        Self { samples }
    }

    /// Predict cost for the next `days` period based on the average of recent samples.
    pub fn forecast(&self, days: u64) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let avg: f64 = self.samples.iter().sum::<f64>() / self.samples.len() as f64;
        avg * days as f64
    }

    /// Suggest a cheaper model if forecast exceeds the budget.
    pub fn suggest_cheaper_model(&self, forecast: f64, budget: f64) -> Option<&'static str> {
        if forecast > budget {
            Some("gpt-4o-mini")
        } else {
            None
        }
    }
}
