//! Quantization-quality tradeoff evaluation for edge models.
//!
//! Evaluates quality degradation introduced by model quantization (e.g., INT8, INT4)
//! relative to a full-precision (FP32) baseline.

/// Quantization format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuantFormat {
    Fp32,
    Fp16,
    Int8,
    Int4,
    Nf4,
    Custom(String),
}

impl std::fmt::Display for QuantFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantFormat::Fp32 => write!(f, "fp32"),
            QuantFormat::Fp16 => write!(f, "fp16"),
            QuantFormat::Int8 => write!(f, "int8"),
            QuantFormat::Int4 => write!(f, "int4"),
            QuantFormat::Nf4 => write!(f, "nf4"),
            QuantFormat::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl QuantFormat {
    /// Bits per weight element.
    pub fn bits(&self) -> u8 {
        match self {
            QuantFormat::Fp32 => 32,
            QuantFormat::Fp16 => 16,
            QuantFormat::Int8 => 8,
            QuantFormat::Int4 | QuantFormat::Nf4 => 4,
            QuantFormat::Custom(_) => 0,
        }
    }

    /// Compression ratio relative to FP32.
    pub fn compression_ratio(&self) -> f64 {
        let b = self.bits();
        if b == 0 {
            return 1.0;
        }
        32.0 / b as f64
    }
}

/// A quality measurement for a given quantization format on a particular task.
#[derive(Debug, Clone)]
pub struct QuantMeasurement {
    pub format: QuantFormat,
    /// Quality score in [0, 1] (higher is better).
    pub quality_score: f64,
    /// Perplexity (lower is better; 0 means not measured).
    pub perplexity: f64,
    /// Memory footprint in MiB.
    pub memory_mib: f64,
}

impl QuantMeasurement {
    pub fn new(format: QuantFormat, quality_score: f64, perplexity: f64, memory_mib: f64) -> Self {
        Self {
            format,
            quality_score: quality_score.clamp(0.0, 1.0),
            perplexity,
            memory_mib,
        }
    }
}

/// Quantization tradeoff evaluator: compares quantized variants against a baseline.
#[derive(Debug)]
pub struct QuantTradeoffEval {
    baseline: QuantMeasurement,
    variants: Vec<QuantMeasurement>,
}

impl QuantTradeoffEval {
    /// Create a new evaluator with a FP32 baseline measurement.
    pub fn new(baseline: QuantMeasurement) -> Self {
        Self {
            baseline,
            variants: Vec::new(),
        }
    }

    /// Add a quantized variant.
    pub fn add_variant(&mut self, variant: QuantMeasurement) {
        self.variants.push(variant);
    }

    /// Compute quality degradation (absolute) compared to baseline.
    pub fn quality_degradation(&self, variant: &QuantMeasurement) -> f64 {
        (self.baseline.quality_score - variant.quality_score).max(0.0)
    }

    /// Compute memory savings ratio (how much smaller vs baseline).
    pub fn memory_savings_ratio(&self, variant: &QuantMeasurement) -> f64 {
        if self.baseline.memory_mib == 0.0 {
            return 0.0;
        }
        (self.baseline.memory_mib - variant.memory_mib).max(0.0) / self.baseline.memory_mib
    }

    /// Returns a tradeoff score: memory_savings / (1 + quality_degradation).
    /// Higher is better.
    pub fn tradeoff_score(&self, variant: &QuantMeasurement) -> f64 {
        let savings = self.memory_savings_ratio(variant);
        let degradation = self.quality_degradation(variant);
        savings / (1.0 + degradation)
    }

    /// Find the best variant by tradeoff score.
    pub fn best_variant(&self) -> Option<&QuantMeasurement> {
        self.variants.iter().max_by(|a, b| {
            self.tradeoff_score(a)
                .partial_cmp(&self.tradeoff_score(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// All variants.
    pub fn variants(&self) -> &[QuantMeasurement] {
        &self.variants
    }

    /// Baseline measurement.
    pub fn baseline(&self) -> &QuantMeasurement {
        &self.baseline
    }
}
