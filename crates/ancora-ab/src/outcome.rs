/// Collects per-variant outcome metric observations.

/// A single outcome observation for one subject in one variant.
#[derive(Debug, Clone)]
pub struct Observation {
    pub experiment_id: String,
    pub subject_key: String,
    pub variant_name: String,
    pub value: f64,
}

impl Observation {
    pub fn new(
        experiment_id: impl Into<String>,
        subject_key: impl Into<String>,
        variant_name: impl Into<String>,
        value: f64,
    ) -> Self {
        Observation {
            experiment_id: experiment_id.into(),
            subject_key: subject_key.into(),
            variant_name: variant_name.into(),
            value,
        }
    }
}

/// Summary statistics for a collection of observations.
#[derive(Debug, Clone)]
pub struct VariantStats {
    pub variant_name: String,
    pub n: usize,
    pub mean: f64,
    pub variance: f64,
}

impl VariantStats {
    /// Standard deviation.
    pub fn std_dev(&self) -> f64 {
        self.variance.sqrt()
    }

    /// Standard error of the mean.
    pub fn std_error(&self) -> f64 {
        if self.n == 0 {
            0.0
        } else {
            self.std_dev() / (self.n as f64).sqrt()
        }
    }
}

/// In-memory store of outcome observations.
#[derive(Debug, Default)]
pub struct OutcomeStore {
    observations: Vec<Observation>,
}

impl OutcomeStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new observation.
    pub fn record(&mut self, obs: Observation) {
        self.observations.push(obs);
    }

    /// Return all observations for a given experiment and variant.
    pub fn values_for_variant(&self, experiment_id: &str, variant_name: &str) -> Vec<f64> {
        self.observations
            .iter()
            .filter(|o| o.experiment_id == experiment_id && o.variant_name == variant_name)
            .map(|o| o.value)
            .collect()
    }

    /// Compute summary statistics for a variant.
    pub fn stats_for_variant(
        &self,
        experiment_id: &str,
        variant_name: &str,
    ) -> Option<VariantStats> {
        let values = self.values_for_variant(experiment_id, variant_name);
        if values.is_empty() {
            return None;
        }
        let n = values.len();
        let mean = values.iter().sum::<f64>() / n as f64;
        let variance = if n == 1 {
            0.0
        } else {
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        };
        Some(VariantStats {
            variant_name: variant_name.to_string(),
            n,
            mean,
            variance,
        })
    }

    /// Return all observations.
    pub fn all(&self) -> &[Observation] {
        &self.observations
    }
}
