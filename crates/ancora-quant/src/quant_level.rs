/// Quantization level metadata.
///
/// Provides a unified abstraction over quantization levels across GGUF and
/// ONNX models, capturing quality/size/speed trade-offs.
use std::fmt;

/// Coarse quantization tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantTier {
    /// Full precision (FP32). Highest quality, largest size.
    Full,
    /// Half precision (FP16 / BF16). Good quality, half the size.
    Half,
    /// 8-bit integer. Small accuracy loss, 4x compression vs FP32.
    Int8,
    /// 5-6 bit. Good balance of size and quality.
    Medium,
    /// 4-bit. Very small, noticeable quality loss.
    Int4,
    /// 2-3 bit. Aggressive compression, significant quality loss.
    Aggressive,
}

impl QuantTier {
    /// Return a short label.
    pub fn label(&self) -> &'static str {
        match self {
            QuantTier::Full => "full",
            QuantTier::Half => "half",
            QuantTier::Int8 => "int8",
            QuantTier::Medium => "medium",
            QuantTier::Int4 => "int4",
            QuantTier::Aggressive => "aggressive",
        }
    }

    /// Approximate compression ratio vs FP32.
    pub fn compression_ratio(&self) -> f32 {
        match self {
            QuantTier::Full => 1.0,
            QuantTier::Half => 2.0,
            QuantTier::Int8 => 4.0,
            QuantTier::Medium => 6.0,
            QuantTier::Int4 => 8.0,
            QuantTier::Aggressive => 12.0,
        }
    }

    /// Relative inference speed multiplier vs FP32 on CPU (higher is faster).
    pub fn cpu_speed_multiplier(&self) -> f32 {
        match self {
            QuantTier::Full => 1.0,
            QuantTier::Half => 1.5,
            QuantTier::Int8 => 2.5,
            QuantTier::Medium => 3.0,
            QuantTier::Int4 => 3.5,
            QuantTier::Aggressive => 4.0,
        }
    }

    /// Estimated perplexity increase vs FP32 (lower is better; 0 = lossless).
    pub fn perplexity_degradation(&self) -> f32 {
        match self {
            QuantTier::Full => 0.0,
            QuantTier::Half => 0.01,
            QuantTier::Int8 => 0.05,
            QuantTier::Medium => 0.15,
            QuantTier::Int4 => 0.35,
            QuantTier::Aggressive => 1.2,
        }
    }
}

impl fmt::Display for QuantTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Detailed quantization level metadata attached to a model.
#[derive(Debug, Clone)]
pub struct QuantLevel {
    /// The coarse tier.
    pub tier: QuantTier,
    /// Exact tag string (e.g. "Q4_K_M", "int8", "Q6_K").
    pub tag: String,
    /// Nominal bits per weight.
    pub bits_per_weight: f32,
    /// Whether this level uses grouped/block quantization (K-quants).
    pub is_grouped: bool,
    /// Human-readable notes on trade-offs.
    pub notes: String,
}

impl QuantLevel {
    /// Construct a new QuantLevel.
    pub fn new(
        tier: QuantTier,
        tag: impl Into<String>,
        bits_per_weight: f32,
        is_grouped: bool,
        notes: impl Into<String>,
    ) -> Self {
        QuantLevel {
            tier,
            tag: tag.into(),
            bits_per_weight,
            is_grouped,
            notes: notes.into(),
        }
    }

    /// Returns a recommended QuantLevel for a given available RAM budget in GB.
    pub fn recommend_for_ram_gb(param_billions: f32, ram_gb: f32) -> QuantTier {
        // Compute bytes needed for each tier with ~10% overhead.
        let tiers = [
            QuantTier::Full,
            QuantTier::Half,
            QuantTier::Int8,
            QuantTier::Medium,
            QuantTier::Int4,
            QuantTier::Aggressive,
        ];
        for &tier in &tiers {
            let bits = match tier {
                QuantTier::Full => 32.0_f32,
                QuantTier::Half => 16.0,
                QuantTier::Int8 => 8.0,
                QuantTier::Medium => 5.5,
                QuantTier::Int4 => 4.5,
                QuantTier::Aggressive => 2.5,
            };
            let gb_needed = param_billions * bits / 8.0 * 1.1;
            if gb_needed <= ram_gb {
                return tier;
            }
        }
        QuantTier::Aggressive
    }
}

impl fmt::Display for QuantLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:.1}bpw)", self.tag, self.bits_per_weight)
    }
}

/// Well-known preset QuantLevels for convenience.
pub mod presets {
    use super::{QuantLevel, QuantTier};

    pub fn q4_k_m() -> QuantLevel {
        QuantLevel::new(
            QuantTier::Int4,
            "Q4_K_M",
            4.5,
            true,
            "Popular 4-bit K-quant. Good quality/size balance for consumer hardware.",
        )
    }

    pub fn q5_k_m() -> QuantLevel {
        QuantLevel::new(
            QuantTier::Medium,
            "Q5_K_M",
            5.0,
            true,
            "5-bit K-quant. Better quality than Q4 with moderate size increase.",
        )
    }

    pub fn q8_0() -> QuantLevel {
        QuantLevel::new(
            QuantTier::Int8,
            "Q8_0",
            8.0,
            false,
            "8-bit quantization. Near-lossless, ~2x compression vs FP16.",
        )
    }

    pub fn fp16() -> QuantLevel {
        QuantLevel::new(
            QuantTier::Half,
            "FP16",
            16.0,
            false,
            "Half precision. Minimal quality loss, requires GPU for practical use.",
        )
    }

    pub fn q2_k() -> QuantLevel {
        QuantLevel::new(
            QuantTier::Aggressive,
            "Q2_K",
            2.5,
            true,
            "Very aggressive 2-bit K-quant. Significant quality loss.",
        )
    }
}
