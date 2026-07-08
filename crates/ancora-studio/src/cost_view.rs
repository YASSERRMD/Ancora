//! Cost analytics view - breakdown of token and API costs per run/step.

#[derive(Debug, Clone)]
pub struct StepCost {
    pub step_index: usize,
    pub model: String,
    pub tokens_in: u32,
    pub tokens_out: u32,
    pub cost_usd: f64,
}

impl StepCost {
    pub fn total_tokens(&self) -> u32 {
        self.tokens_in + self.tokens_out
    }
}

#[derive(Debug, Clone)]
pub struct CostBreakdown {
    pub run_id: String,
    pub steps: Vec<StepCost>,
}

impl CostBreakdown {
    pub fn new(run_id: impl Into<String>, steps: Vec<StepCost>) -> Self {
        Self {
            run_id: run_id.into(),
            steps,
        }
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.steps.iter().map(|s| s.cost_usd).sum()
    }

    pub fn total_tokens_in(&self) -> u32 {
        self.steps.iter().map(|s| s.tokens_in).sum()
    }

    pub fn total_tokens_out(&self) -> u32 {
        self.steps.iter().map(|s| s.tokens_out).sum()
    }

    pub fn cost_by_model(&self) -> std::collections::HashMap<String, f64> {
        let mut map: std::collections::HashMap<String, f64> = Default::default();
        for step in &self.steps {
            *map.entry(step.model.clone()).or_insert(0.0) += step.cost_usd;
        }
        map
    }

    pub fn most_expensive_step(&self) -> Option<&StepCost> {
        self.steps.iter().max_by(|a, b| {
            a.cost_usd
                .partial_cmp(&b.cost_usd)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn average_cost_per_step(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.total_cost_usd() / self.steps.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_breakdown() -> CostBreakdown {
        CostBreakdown::new(
            "r1",
            vec![
                StepCost {
                    step_index: 0,
                    model: "gpt-4".into(),
                    tokens_in: 100,
                    tokens_out: 50,
                    cost_usd: 0.003,
                },
                StepCost {
                    step_index: 1,
                    model: "gpt-3.5".into(),
                    tokens_in: 50,
                    tokens_out: 20,
                    cost_usd: 0.001,
                },
            ],
        )
    }

    #[test]
    fn test_total_cost() {
        let b = sample_breakdown();
        assert!((b.total_cost_usd() - 0.004).abs() < 1e-9);
    }

    #[test]
    fn test_cost_by_model() {
        let b = sample_breakdown();
        let by_model = b.cost_by_model();
        assert!((by_model["gpt-4"] - 0.003).abs() < 1e-9);
    }

    #[test]
    fn test_most_expensive_step() {
        let b = sample_breakdown();
        let step = b.most_expensive_step().unwrap();
        assert_eq!(step.step_index, 0);
    }
}
