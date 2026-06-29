/// Quantization-aware capability flags for local models.
///
/// Tracks what tasks a quantized model supports and any limitations
/// introduced by quantization.
use std::collections::HashSet;

/// Task capabilities that a model may support.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Text generation / completion.
    TextGeneration,
    /// Chat / instruction following.
    Chat,
    /// Embedding generation.
    Embedding,
    /// Text classification.
    Classification,
    /// Question answering.
    QuestionAnswering,
    /// Code generation.
    CodeGeneration,
    /// Tool / function calling.
    ToolCalling,
    /// Multi-modal (vision) input.
    Vision,
    /// Text-to-speech.
    TextToSpeech,
    /// Speech-to-text.
    SpeechToText,
    /// Reranking.
    Reranking,
    /// Custom capability.
    Custom(String),
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Capability::TextGeneration => "text-generation",
            Capability::Chat => "chat",
            Capability::Embedding => "embedding",
            Capability::Classification => "classification",
            Capability::QuestionAnswering => "question-answering",
            Capability::CodeGeneration => "code-generation",
            Capability::ToolCalling => "tool-calling",
            Capability::Vision => "vision",
            Capability::TextToSpeech => "text-to-speech",
            Capability::SpeechToText => "speech-to-text",
            Capability::Reranking => "reranking",
            Capability::Custom(c) => c.as_str(),
        };
        write!(f, "{}", s)
    }
}

/// Limitations imposed by quantization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantLimitations {
    /// Maximum recommended context length (shorter than training context due to KV cache RAM).
    pub max_context_length: Option<u32>,
    /// Whether batch size > 1 is reliably supported.
    pub supports_batching: bool,
    /// Whether the model can run on CPU at reasonable speed.
    pub cpu_viable: bool,
    /// Minimum recommended GPU VRAM in MB (0 if CPU-only).
    pub min_vram_mb: u32,
    /// Additional notes.
    pub notes: String,
}

impl Default for QuantLimitations {
    fn default() -> Self {
        QuantLimitations {
            max_context_length: None,
            supports_batching: true,
            cpu_viable: true,
            min_vram_mb: 0,
            notes: String::new(),
        }
    }
}

/// Capability flags for a quantized model.
#[derive(Debug, Clone, Default)]
pub struct CapabilityFlags {
    pub capabilities: HashSet<Capability>,
    pub limitations: QuantLimitations,
}

impl CapabilityFlags {
    /// Create an empty capability set.
    pub fn new() -> Self {
        CapabilityFlags::default()
    }

    /// Add a capability.
    pub fn with_capability(mut self, cap: Capability) -> Self {
        self.capabilities.insert(cap);
        self
    }

    /// Set limitations.
    pub fn with_limitations(mut self, limits: QuantLimitations) -> Self {
        self.limitations = limits;
        self
    }

    /// Check whether a capability is supported.
    pub fn supports(&self, cap: &Capability) -> bool {
        self.capabilities.contains(cap)
    }

    /// Add multiple capabilities at once.
    pub fn add_capability(&mut self, cap: Capability) {
        self.capabilities.insert(cap);
    }

    /// Remove a capability (e.g. because quantization degraded it below threshold).
    pub fn remove_capability(&mut self, cap: &Capability) {
        self.capabilities.remove(cap);
    }

    /// Effective max context length, respecting quantization limits.
    pub fn effective_context_length(&self, model_native: u32) -> u32 {
        self.limitations
            .max_context_length
            .unwrap_or(model_native)
            .min(model_native)
    }

    /// List all capabilities as strings.
    pub fn capability_names(&self) -> Vec<String> {
        let mut v: Vec<String> = self.capabilities.iter().map(|c| c.to_string()).collect();
        v.sort();
        v
    }
}

/// Builder for common capability profiles.
pub struct CapabilityBuilder;

impl CapabilityBuilder {
    /// Standard chat model capabilities.
    pub fn chat_model(cpu_viable: bool, min_vram_mb: u32) -> CapabilityFlags {
        CapabilityFlags::new()
            .with_capability(Capability::TextGeneration)
            .with_capability(Capability::Chat)
            .with_limitations(QuantLimitations {
                cpu_viable,
                min_vram_mb,
                ..Default::default()
            })
    }

    /// Embedding model capabilities.
    pub fn embedding_model() -> CapabilityFlags {
        CapabilityFlags::new()
            .with_capability(Capability::Embedding)
            .with_limitations(QuantLimitations {
                cpu_viable: true,
                supports_batching: true,
                ..Default::default()
            })
    }

    /// Code generation model.
    pub fn code_model(cpu_viable: bool) -> CapabilityFlags {
        CapabilityFlags::new()
            .with_capability(Capability::TextGeneration)
            .with_capability(Capability::Chat)
            .with_capability(Capability::CodeGeneration)
            .with_limitations(QuantLimitations {
                cpu_viable,
                ..Default::default()
            })
    }
}
