//! Small-model capability evaluation suite.
//!
//! Provides structures and logic to evaluate small language models (SLMs)
//! against a set of capability benchmarks tuned for edge constraints.

/// A task category used in small-model capability evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskCategory {
    Reasoning,
    Summarization,
    Classification,
    Extraction,
    Qa,
}

impl std::fmt::Display for TaskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskCategory::Reasoning => write!(f, "reasoning"),
            TaskCategory::Summarization => write!(f, "summarization"),
            TaskCategory::Classification => write!(f, "classification"),
            TaskCategory::Extraction => write!(f, "extraction"),
            TaskCategory::Qa => write!(f, "qa"),
        }
    }
}

/// A single capability sample used in an evaluation suite.
#[derive(Debug, Clone)]
pub struct CapabilitySample {
    pub id: String,
    pub category: TaskCategory,
    pub prompt: String,
    pub expected: String,
}

impl CapabilitySample {
    pub fn new(
        id: impl Into<String>,
        category: TaskCategory,
        prompt: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            category,
            prompt: prompt.into(),
            expected: expected.into(),
        }
    }
}

/// Result of evaluating one sample.
#[derive(Debug, Clone)]
pub struct SampleResult {
    pub sample_id: String,
    pub category: TaskCategory,
    pub passed: bool,
    pub score: f64,
}

impl SampleResult {
    pub fn new(sample_id: impl Into<String>, category: TaskCategory, passed: bool, score: f64) -> Self {
        let score = score.clamp(0.0, 1.0);
        Self {
            sample_id: sample_id.into(),
            category,
            passed,
            score,
        }
    }
}

/// A suite of capability samples for small-model evaluation.
#[derive(Debug, Default)]
pub struct SmallModelSuite {
    samples: Vec<CapabilitySample>,
}

impl SmallModelSuite {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a sample to the suite.
    pub fn add(&mut self, sample: CapabilitySample) {
        self.samples.push(sample);
    }

    /// Return all samples.
    pub fn samples(&self) -> &[CapabilitySample] {
        &self.samples
    }

    /// Evaluate using a simple exact-match scorer (for tests and offline use).
    pub fn evaluate_exact(&self, outputs: &[(&str, &str)]) -> Vec<SampleResult> {
        let mut results = Vec::new();
        for sample in &self.samples {
            let output = outputs.iter().find(|(id, _)| *id == sample.id).map(|(_, o)| *o);
            let (passed, score) = match output {
                Some(o) => {
                    let passed = o.trim().to_lowercase() == sample.expected.trim().to_lowercase();
                    let score = if passed { 1.0 } else { 0.0 };
                    (passed, score)
                }
                None => (false, 0.0),
            };
            results.push(SampleResult::new(sample.id.clone(), sample.category.clone(), passed, score));
        }
        results
    }

    /// Aggregate pass rate across all results.
    pub fn pass_rate(results: &[SampleResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        let passed = results.iter().filter(|r| r.passed).count();
        passed as f64 / results.len() as f64
    }

    /// Aggregate mean score across all results.
    pub fn mean_score(results: &[SampleResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        results.iter().map(|r| r.score).sum::<f64>() / results.len() as f64
    }
}

/// A named small model descriptor.
#[derive(Debug, Clone)]
pub struct SmallModel {
    pub name: String,
    pub param_count_millions: u64,
    pub quantization_bits: u8,
}

impl SmallModel {
    pub fn new(name: impl Into<String>, param_count_millions: u64, quantization_bits: u8) -> Self {
        Self {
            name: name.into(),
            param_count_millions,
            quantization_bits,
        }
    }

    /// Returns true if the model is within typical SLM bounds (<= 7B params).
    pub fn is_slm(&self) -> bool {
        self.param_count_millions <= 7_000
    }
}
