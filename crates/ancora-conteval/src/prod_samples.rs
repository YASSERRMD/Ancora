//! Production sample management for continuous evaluation.
//!
//! Draws samples from live production traffic (with redaction applied)
//! to form an evaluation set that reflects real usage patterns.

/// Sensitivity classification for a sample.
#[derive(Debug, Clone, PartialEq)]
pub enum Sensitivity {
    /// No PII or sensitive content detected.
    Public,
    /// Contains PII that must be redacted before evaluation.
    Pii,
    /// Redacted version is safe for evaluation.
    Redacted,
}

/// A single production traffic sample.
#[derive(Debug, Clone)]
pub struct ProdSample {
    pub id: String,
    pub model: String,
    pub provider: String,
    pub prompt: String,
    pub response: String,
    pub latency_ms: u64,
    pub sensitivity: Sensitivity,
}

impl ProdSample {
    pub fn new(
        id: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
        prompt: impl Into<String>,
        response: impl Into<String>,
        latency_ms: u64,
    ) -> Self {
        ProdSample {
            id: id.into(),
            model: model.into(),
            provider: provider.into(),
            prompt: prompt.into(),
            response: response.into(),
            latency_ms,
            sensitivity: Sensitivity::Public,
        }
    }

    /// Mark this sample as containing PII.
    pub fn with_pii(mut self) -> Self {
        self.sensitivity = Sensitivity::Pii;
        self
    }

    /// Apply redaction: replace sensitive text with a placeholder.
    pub fn redact(&mut self, placeholder: &str) {
        if self.sensitivity == Sensitivity::Pii {
            self.prompt = placeholder.to_string();
            self.response = placeholder.to_string();
            self.sensitivity = Sensitivity::Redacted;
        }
    }

    /// Returns true if this sample is safe for evaluation.
    pub fn is_eval_safe(&self) -> bool {
        matches!(
            self.sensitivity,
            Sensitivity::Public | Sensitivity::Redacted
        )
    }
}

/// A collection of production samples used as an evaluation set.
#[derive(Debug, Default)]
pub struct ProdEvalSet {
    samples: Vec<ProdSample>,
}

impl ProdEvalSet {
    pub fn new() -> Self {
        ProdEvalSet {
            samples: Vec::new(),
        }
    }

    /// Add a sample to the set. Returns an error if the sample contains
    /// un-redacted PII.
    pub fn add(&mut self, sample: ProdSample) -> Result<(), String> {
        if sample.sensitivity == Sensitivity::Pii {
            return Err(format!(
                "sample '{}' has un-redacted PII - redact before adding",
                sample.id
            ));
        }
        self.samples.push(sample);
        Ok(())
    }

    /// Return all eval-safe samples.
    pub fn safe_samples(&self) -> Vec<&ProdSample> {
        self.samples.iter().filter(|s| s.is_eval_safe()).collect()
    }

    /// Return the total number of samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns true if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Filter samples by model name.
    pub fn by_model(&self, model: &str) -> Vec<&ProdSample> {
        self.samples.iter().filter(|s| s.model == model).collect()
    }

    /// Filter samples by provider.
    pub fn by_provider(&self, provider: &str) -> Vec<&ProdSample> {
        self.samples
            .iter()
            .filter(|s| s.provider == provider)
            .collect()
    }
}
