/// ONNX model file descriptor.
///
/// ONNX (Open Neural Network Exchange) is a cross-platform model format
/// supported by runtimes such as ONNX Runtime, TensorRT, and DirectML.
use std::collections::HashMap;
use std::path::PathBuf;

/// Precision / element type of ONNX model weights.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnnxPrecision {
    /// 32-bit floating point (full precision).
    Float32,
    /// 16-bit floating point.
    Float16,
    /// bfloat16.
    BFloat16,
    /// 8-bit integer (INT8 quantized).
    Int8,
    /// 4-bit quantized (via QDQ operators or external quantization).
    Int4,
    /// Mixed precision: some layers at a higher precision.
    Mixed,
}

impl OnnxPrecision {
    /// Nominal bits per element.
    pub fn bits(&self) -> u8 {
        match self {
            OnnxPrecision::Float32 => 32,
            OnnxPrecision::Float16 | OnnxPrecision::BFloat16 => 16,
            OnnxPrecision::Int8 => 8,
            OnnxPrecision::Int4 => 4,
            OnnxPrecision::Mixed => 16, // conservative estimate
        }
    }

    /// Parse from a short tag string.
    pub fn from_tag(tag: &str) -> Self {
        match tag.to_lowercase().as_str() {
            "fp32" | "float32" => OnnxPrecision::Float32,
            "fp16" | "float16" => OnnxPrecision::Float16,
            "bf16" | "bfloat16" => OnnxPrecision::BFloat16,
            "int8" | "i8" => OnnxPrecision::Int8,
            "int4" | "i4" => OnnxPrecision::Int4,
            _ => OnnxPrecision::Mixed,
        }
    }
}

impl std::fmt::Display for OnnxPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OnnxPrecision::Float32 => "fp32",
            OnnxPrecision::Float16 => "fp16",
            OnnxPrecision::BFloat16 => "bf16",
            OnnxPrecision::Int8 => "int8",
            OnnxPrecision::Int4 => "int4",
            OnnxPrecision::Mixed => "mixed",
        };
        write!(f, "{}", s)
    }
}

/// Execution provider hints for ONNX Runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionProvider {
    Cpu,
    Cuda,
    TensorRT,
    DirectML,
    CoreML,
    OpenVino,
    Custom(String),
}

impl std::fmt::Display for ExecutionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExecutionProvider::Cpu => "cpu",
            ExecutionProvider::Cuda => "cuda",
            ExecutionProvider::TensorRT => "tensorrt",
            ExecutionProvider::DirectML => "directml",
            ExecutionProvider::CoreML => "coreml",
            ExecutionProvider::OpenVino => "openvino",
            ExecutionProvider::Custom(c) => c.as_str(),
        };
        write!(f, "{}", s)
    }
}

/// Descriptor for an ONNX model.
#[derive(Debug, Clone)]
pub struct OnnxDescriptor {
    /// Human-readable model name.
    pub name: String,
    /// Local file path to the .onnx file.
    pub path: PathBuf,
    /// ONNX opset version.
    pub opset_version: u32,
    /// Model precision / quantization level.
    pub precision: OnnxPrecision,
    /// Preferred execution providers in priority order.
    pub providers: Vec<ExecutionProvider>,
    /// File size in bytes.
    pub file_size_bytes: u64,
    /// Number of model parameters in billions (approximate).
    pub param_count_billions: f32,
    /// Maximum sequence length supported (if applicable).
    pub max_sequence_length: Option<u32>,
    /// Extra metadata.
    pub metadata: HashMap<String, String>,
}

impl OnnxDescriptor {
    /// Create a new ONNX descriptor.
    pub fn new(
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        opset_version: u32,
        precision: OnnxPrecision,
        file_size_bytes: u64,
        param_count_billions: f32,
    ) -> Self {
        OnnxDescriptor {
            name: name.into(),
            path: path.into(),
            opset_version,
            precision,
            providers: vec![ExecutionProvider::Cpu],
            file_size_bytes,
            param_count_billions,
            max_sequence_length: None,
            metadata: HashMap::new(),
        }
    }

    /// Add an execution provider (appended at lower priority).
    pub fn with_provider(mut self, provider: ExecutionProvider) -> Self {
        self.providers.push(provider);
        self
    }

    /// Set max sequence length.
    pub fn with_max_sequence_length(mut self, len: u32) -> Self {
        self.max_sequence_length = Some(len);
        self
    }

    /// Insert a metadata key/value pair.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Estimated RAM requirement in bytes.
    pub fn estimated_ram_bytes(&self) -> u64 {
        let params = (self.param_count_billions * 1e9) as u64;
        let raw = params * self.precision.bits() as u64 / 8;
        raw + raw / 10
    }

    /// Return the file extension (always "onnx").
    pub fn extension(&self) -> &str {
        "onnx"
    }
}
