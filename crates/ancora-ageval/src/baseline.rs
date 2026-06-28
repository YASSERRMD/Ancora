//! Regression baseline storage: compare current scores against stored baselines.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BaselineResult {
    Passed { score: f64 },
    Regressed { expected: f64, actual: f64, delta: f64 },
    NoPrior,
}

pub struct BaselineStore {
    baselines: HashMap<String, f64>,
    /// Allowed downward delta before a regression is declared.
    tolerance: f64,
}

impl BaselineStore {
    pub fn new(tolerance: f64) -> Self {
        Self {
            baselines: HashMap::new(),
            tolerance,
        }
    }

    pub fn set(&mut self, metric: &str, score: f64) {
        self.baselines.insert(metric.to_string(), score);
    }

    pub fn get(&self, metric: &str) -> Option<f64> {
        self.baselines.get(metric).copied()
    }

    pub fn check(&self, metric: &str, score: f64) -> BaselineResult {
        match self.baselines.get(metric) {
            None => BaselineResult::NoPrior,
            Some(&baseline) => {
                let delta = score - baseline;
                if delta >= -self.tolerance {
                    BaselineResult::Passed { score }
                } else {
                    BaselineResult::Regressed {
                        expected: baseline,
                        actual: score,
                        delta,
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.baselines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.baselines.is_empty()
    }
}
