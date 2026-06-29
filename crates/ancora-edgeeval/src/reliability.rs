//! SLM reliability evaluation.
//!
//! Evaluates small language models for output reliability, including consistency
//! across repeated runs, graceful failure modes, and confidence calibration.

/// A reliability scenario for SLM evaluation.
#[derive(Debug, Clone)]
pub struct ReliabilityScenario {
    pub id: String,
    pub description: String,
    /// Minimum acceptable reliability score in [0, 1].
    pub threshold: f64,
}

impl ReliabilityScenario {
    pub fn new(id: impl Into<String>, description: impl Into<String>, threshold: f64) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            threshold: threshold.clamp(0.0, 1.0),
        }
    }
}

/// The result of running a reliability scenario.
#[derive(Debug, Clone)]
pub struct ReliabilityResult {
    pub scenario_id: String,
    /// Score in [0, 1].
    pub score: f64,
    /// Whether the score met the threshold.
    pub passed: bool,
    pub notes: Option<String>,
}

impl ReliabilityResult {
    pub fn new(scenario_id: impl Into<String>, score: f64, threshold: f64) -> Self {
        let score = score.clamp(0.0, 1.0);
        Self {
            scenario_id: scenario_id.into(),
            score,
            passed: score >= threshold,
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Measures output consistency across multiple runs (same prompt, multiple outputs).
pub struct ConsistencyChecker;

impl ConsistencyChecker {
    /// Compute consistency score: fraction of outputs matching the plurality output.
    pub fn score(outputs: &[&str]) -> f64 {
        if outputs.is_empty() {
            return 0.0;
        }
        if outputs.len() == 1 {
            return 1.0;
        }
        // Count occurrences of each unique output.
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for &o in outputs {
            *counts.entry(o).or_insert(0) += 1;
        }
        let max_count = counts.values().copied().max().unwrap_or(0);
        max_count as f64 / outputs.len() as f64
    }
}

/// Measures confidence calibration: whether stated confidence aligns with actual accuracy.
pub struct CalibrationEval;

impl CalibrationEval {
    /// ECE (Expected Calibration Error) approximation for binary correct/incorrect with stated confidence.
    /// Lower is better. Returns a score in [0, 1] where 0 = perfect calibration.
    pub fn ece(pairs: &[(f64, bool)]) -> f64 {
        if pairs.is_empty() {
            return 0.0;
        }
        // Simple binning into 10 buckets.
        let n_bins = 10usize;
        let mut bin_total = vec![0usize; n_bins];
        let mut bin_correct = vec![0usize; n_bins];
        let mut bin_conf_sum = vec![0.0f64; n_bins];

        for &(conf, correct) in pairs {
            let conf = conf.clamp(0.0, 1.0);
            let bin = ((conf * n_bins as f64) as usize).min(n_bins - 1);
            bin_total[bin] += 1;
            if correct {
                bin_correct[bin] += 1;
            }
            bin_conf_sum[bin] += conf;
        }

        let total = pairs.len() as f64;
        let mut ece = 0.0;
        for b in 0..n_bins {
            if bin_total[b] == 0 {
                continue;
            }
            let acc = bin_correct[b] as f64 / bin_total[b] as f64;
            let avg_conf = bin_conf_sum[b] / bin_total[b] as f64;
            ece += (bin_total[b] as f64 / total) * (acc - avg_conf).abs();
        }
        ece
    }
}

/// SLM reliability evaluator.
#[derive(Debug, Default)]
pub struct SlmReliabilityEval {
    results: Vec<ReliabilityResult>,
}

impl SlmReliabilityEval {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a reliability result.
    pub fn add_result(&mut self, result: ReliabilityResult) {
        self.results.push(result);
    }

    /// Overall reliability score (mean of all scenario scores).
    pub fn overall_score(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        self.results.iter().map(|r| r.score).sum::<f64>() / self.results.len() as f64
    }

    /// Pass rate across all scenarios.
    pub fn pass_rate(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        let passed = self.results.iter().filter(|r| r.passed).count();
        passed as f64 / self.results.len() as f64
    }

    /// All results.
    pub fn results(&self) -> &[ReliabilityResult] {
        &self.results
    }
}
