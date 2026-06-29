//! Offline-only eval mode for edge evaluation.
//!
//! Ensures that all evaluation steps run without any network access.
//! Provides static datasets and deterministic mock scorers for offline testing.

/// Offline eval configuration.
#[derive(Debug, Clone)]
pub struct OfflineConfig {
    /// Whether to strictly disallow any network I/O (checked at runtime via flag).
    pub strict_offline: bool,
    /// Maximum number of samples to evaluate per suite.
    pub max_samples: usize,
    /// Seed for deterministic scoring in mock mode.
    pub seed: u64,
}

impl Default for OfflineConfig {
    fn default() -> Self {
        Self {
            strict_offline: true,
            max_samples: 100,
            seed: 42,
        }
    }
}

impl OfflineConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_strict_offline(mut self, strict: bool) -> Self {
        self.strict_offline = strict;
        self
    }

    pub fn with_max_samples(mut self, n: usize) -> Self {
        self.max_samples = n;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}

/// A static offline dataset: pre-loaded samples with ground truth.
#[derive(Debug, Clone)]
pub struct OfflineSample {
    pub id: String,
    pub input: String,
    pub ground_truth: String,
}

impl OfflineSample {
    pub fn new(id: impl Into<String>, input: impl Into<String>, ground_truth: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            input: input.into(),
            ground_truth: ground_truth.into(),
        }
    }
}

/// A static offline dataset collection.
#[derive(Debug, Default)]
pub struct OfflineDataset {
    samples: Vec<OfflineSample>,
}

impl OfflineDataset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, sample: OfflineSample) {
        self.samples.push(sample);
    }

    pub fn samples(&self) -> &[OfflineSample] {
        &self.samples
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Return a standard built-in offline dataset for smoke testing.
    pub fn builtin_smoke() -> Self {
        let mut ds = Self::new();
        ds.add(OfflineSample::new("s1", "What is 2+2?", "4"));
        ds.add(OfflineSample::new("s2", "Classify: 'Great product!' -> positive/negative", "positive"));
        ds.add(OfflineSample::new("s3", "Extract the city: 'Meeting in Paris next week.'", "Paris"));
        ds.add(OfflineSample::new("s4", "True or False: The sky is blue.", "True"));
        ds.add(OfflineSample::new("s5", "Summarize in one word: 'The quick brown fox jumps.'", "fox"));
        ds
    }
}

/// A deterministic mock scorer for offline evaluation.
/// Uses a simple hash of the input and seed to produce scores.
pub struct MockScorer {
    seed: u64,
}

impl MockScorer {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Score an output against ground truth deterministically.
    pub fn score(&self, output: &str, ground_truth: &str) -> f64 {
        if output.trim().to_lowercase() == ground_truth.trim().to_lowercase() {
            1.0
        } else {
            // Deterministic partial score based on character overlap.
            let o = output.trim().to_lowercase();
            let g = ground_truth.trim().to_lowercase();
            let common = o.chars().filter(|c| g.contains(*c)).count();
            let max_len = o.len().max(g.len()).max(1);
            let overlap = common as f64 / max_len as f64;
            // Mix with seed to make it deterministic but varied.
            let noise = (self.seed % 10) as f64 / 100.0;
            (overlap + noise).clamp(0.0, 1.0)
        }
    }
}

/// Offline eval runner: runs evaluation entirely from static data.
pub struct OfflineEvalRunner {
    pub config: OfflineConfig,
}

impl OfflineEvalRunner {
    pub fn new(config: OfflineConfig) -> Self {
        Self { config }
    }

    /// Run evaluation on a dataset with provided outputs (offline).
    /// Returns (sample_id, score) pairs.
    pub fn run(
        &self,
        dataset: &OfflineDataset,
        outputs: &[(&str, &str)],
    ) -> Vec<(String, f64)> {
        let scorer = MockScorer::new(self.config.seed);
        dataset
            .samples()
            .iter()
            .take(self.config.max_samples)
            .map(|s| {
                let output = outputs.iter().find(|(id, _)| *id == s.id).map(|(_, o)| *o).unwrap_or("");
                let score = scorer.score(output, &s.ground_truth);
                (s.id.clone(), score)
            })
            .collect()
    }
}
