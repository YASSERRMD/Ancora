/// GGUF model file descriptor.
///
/// GGUF (GPT-Generated Unified Format) is the file format used by llama.cpp
/// and compatible runtimes for storing quantized language models.
use std::collections::HashMap;
use std::path::PathBuf;

/// Quantization type for a GGUF tensor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GgufQuantType {
    F32,
    F16,
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q8_1,
    Q2_K,
    Q3_K,
    Q4_K,
    Q5_K,
    Q6_K,
    Q8_K,
    IQ2_XXS,
    IQ2_XS,
    IQ3_XXS,
    IQ3_XS,
    IQ4_NL,
    IQ4_XS,
    BF16,
    Other(String),
}

impl GgufQuantType {
    /// Parse from a string tag, e.g. "Q4_K_M" -> Q4_K.
    pub fn from_tag(tag: &str) -> Self {
        match tag.to_uppercase().as_str() {
            "F32" => GgufQuantType::F32,
            "F16" => GgufQuantType::F16,
            "BF16" => GgufQuantType::BF16,
            s if s.starts_with("Q4_0") => GgufQuantType::Q4_0,
            s if s.starts_with("Q4_1") => GgufQuantType::Q4_1,
            s if s.starts_with("Q5_0") => GgufQuantType::Q5_0,
            s if s.starts_with("Q5_1") => GgufQuantType::Q5_1,
            s if s.starts_with("Q8_0") => GgufQuantType::Q8_0,
            s if s.starts_with("Q8_1") => GgufQuantType::Q8_1,
            s if s.starts_with("Q2_K") => GgufQuantType::Q2_K,
            s if s.starts_with("Q3_K") => GgufQuantType::Q3_K,
            s if s.starts_with("Q4_K") => GgufQuantType::Q4_K,
            s if s.starts_with("Q5_K") => GgufQuantType::Q5_K,
            s if s.starts_with("Q6_K") => GgufQuantType::Q6_K,
            s if s.starts_with("Q8_K") => GgufQuantType::Q8_K,
            s if s.starts_with("IQ2_XXS") => GgufQuantType::IQ2_XXS,
            s if s.starts_with("IQ2_XS") => GgufQuantType::IQ2_XS,
            s if s.starts_with("IQ3_XXS") => GgufQuantType::IQ3_XXS,
            s if s.starts_with("IQ3_XS") => GgufQuantType::IQ3_XS,
            s if s.starts_with("IQ4_NL") => GgufQuantType::IQ4_NL,
            s if s.starts_with("IQ4_XS") => GgufQuantType::IQ4_XS,
            other => GgufQuantType::Other(other.to_string()),
        }
    }

    /// Approximate bits-per-weight for this quant type.
    pub fn bits_per_weight(&self) -> f32 {
        match self {
            GgufQuantType::F32 => 32.0,
            GgufQuantType::F16 | GgufQuantType::BF16 => 16.0,
            GgufQuantType::Q8_0 | GgufQuantType::Q8_1 | GgufQuantType::Q8_K => 8.0,
            GgufQuantType::Q6_K => 6.5,
            GgufQuantType::Q5_0 | GgufQuantType::Q5_1 | GgufQuantType::Q5_K => 5.0,
            GgufQuantType::Q4_0 | GgufQuantType::Q4_1 | GgufQuantType::Q4_K => 4.5,
            GgufQuantType::IQ4_NL | GgufQuantType::IQ4_XS => 4.0,
            GgufQuantType::Q3_K => 3.5,
            GgufQuantType::IQ3_XXS | GgufQuantType::IQ3_XS => 3.0,
            GgufQuantType::Q2_K => 2.5,
            GgufQuantType::IQ2_XXS | GgufQuantType::IQ2_XS => 2.0,
            GgufQuantType::Other(_) => 8.0,
        }
    }
}

impl std::fmt::Display for GgufQuantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GgufQuantType::F32 => "F32",
            GgufQuantType::F16 => "F16",
            GgufQuantType::BF16 => "BF16",
            GgufQuantType::Q4_0 => "Q4_0",
            GgufQuantType::Q4_1 => "Q4_1",
            GgufQuantType::Q5_0 => "Q5_0",
            GgufQuantType::Q5_1 => "Q5_1",
            GgufQuantType::Q8_0 => "Q8_0",
            GgufQuantType::Q8_1 => "Q8_1",
            GgufQuantType::Q2_K => "Q2_K",
            GgufQuantType::Q3_K => "Q3_K",
            GgufQuantType::Q4_K => "Q4_K",
            GgufQuantType::Q5_K => "Q5_K",
            GgufQuantType::Q6_K => "Q6_K",
            GgufQuantType::Q8_K => "Q8_K",
            GgufQuantType::IQ2_XXS => "IQ2_XXS",
            GgufQuantType::IQ2_XS => "IQ2_XS",
            GgufQuantType::IQ3_XXS => "IQ3_XXS",
            GgufQuantType::IQ3_XS => "IQ3_XS",
            GgufQuantType::IQ4_NL => "IQ4_NL",
            GgufQuantType::IQ4_XS => "IQ4_XS",
            GgufQuantType::Other(s) => s.as_str(),
        };
        write!(f, "{}", s)
    }
}

/// Descriptor for a GGUF model file.
#[derive(Debug, Clone)]
pub struct GgufDescriptor {
    /// Human-readable model name.
    pub name: String,
    /// Local file path.
    pub path: PathBuf,
    /// GGUF format version.
    pub gguf_version: u32,
    /// Architecture name (e.g. "llama", "mistral").
    pub architecture: String,
    /// Number of model parameters in billions.
    pub param_count_billions: f32,
    /// Primary quantization type used for weight tensors.
    pub quant_type: GgufQuantType,
    /// File size in bytes.
    pub file_size_bytes: u64,
    /// Context length supported.
    pub context_length: u32,
    /// Extra key/value metadata from the GGUF header.
    pub metadata: HashMap<String, String>,
}

impl GgufDescriptor {
    /// Create a new GGUF descriptor.
    pub fn new(
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        architecture: impl Into<String>,
        param_count_billions: f32,
        quant_type: GgufQuantType,
        file_size_bytes: u64,
        context_length: u32,
    ) -> Self {
        GgufDescriptor {
            name: name.into(),
            path: path.into(),
            gguf_version: 3,
            architecture: architecture.into(),
            param_count_billions,
            quant_type,
            file_size_bytes,
            context_length,
            metadata: HashMap::new(),
        }
    }

    /// Estimated RAM requirement in bytes to load this model.
    /// Uses bits-per-weight and parameter count; adds ~10% overhead.
    pub fn estimated_ram_bytes(&self) -> u64 {
        let bits = self.quant_type.bits_per_weight();
        let params = (self.param_count_billions * 1e9) as u64;
        let raw = (params as f64 * bits as f64 / 8.0) as u64;
        // Add 10% overhead for KV cache, context buffer, etc.
        raw + raw / 10
    }

    /// Insert a metadata key/value pair.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Return the file extension (always ".gguf" for valid descriptors).
    pub fn extension(&self) -> &str {
        "gguf"
    }
}
