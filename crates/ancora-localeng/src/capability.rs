/// Engine capability detection.
///
/// Each engine supports a different subset of features.  This module
/// provides a uniform `Capabilities` struct and detection helpers so
/// callers can choose the right engine at runtime.

use crate::model::EngineKind;

/// A set of features an engine may or may not support.
#[derive(Debug, Clone, PartialEq)]
pub struct Capabilities {
    pub streaming: bool,
    pub embeddings: bool,
    pub vision: bool,
    pub function_calling: bool,
    pub batch_inference: bool,
    pub speculative_decoding: bool,
    pub continuous_batching: bool,
    pub quantization: bool,
    pub lora_adapters: bool,
    pub grammar_constrained: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            streaming: false,
            embeddings: false,
            vision: false,
            function_calling: false,
            batch_inference: false,
            speculative_decoding: false,
            continuous_batching: false,
            quantization: false,
            lora_adapters: false,
            grammar_constrained: false,
        }
    }
}

impl Capabilities {
    /// Returns well-known capabilities for each built-in engine kind.
    pub fn for_engine(kind: &EngineKind) -> Self {
        match kind {
            EngineKind::LlamaCppServer => Capabilities {
                streaming: true,
                embeddings: true,
                vision: false,
                function_calling: false,
                batch_inference: false,
                speculative_decoding: false,
                continuous_batching: false,
                quantization: true,
                lora_adapters: true,
                grammar_constrained: true,
            },
            EngineKind::LlamaCppEmbedded => Capabilities {
                streaming: false,
                embeddings: true,
                vision: false,
                function_calling: false,
                batch_inference: false,
                speculative_decoding: false,
                continuous_batching: false,
                quantization: true,
                lora_adapters: false,
                grammar_constrained: true,
            },
            EngineKind::Ollama => Capabilities {
                streaming: true,
                embeddings: true,
                vision: true,
                function_calling: true,
                batch_inference: false,
                speculative_decoding: false,
                continuous_batching: false,
                quantization: true,
                lora_adapters: false,
                grammar_constrained: false,
            },
            EngineKind::Vllm => Capabilities {
                streaming: true,
                embeddings: false,
                vision: false,
                function_calling: true,
                batch_inference: true,
                speculative_decoding: true,
                continuous_batching: true,
                quantization: true,
                lora_adapters: true,
                grammar_constrained: false,
            },
            EngineKind::Sglang => Capabilities {
                streaming: true,
                embeddings: false,
                vision: false,
                function_calling: true,
                batch_inference: true,
                speculative_decoding: true,
                continuous_batching: true,
                quantization: true,
                lora_adapters: true,
                grammar_constrained: true,
            },
            EngineKind::LmStudio => Capabilities {
                streaming: true,
                embeddings: true,
                vision: false,
                function_calling: true,
                batch_inference: false,
                speculative_decoding: false,
                continuous_batching: false,
                quantization: true,
                lora_adapters: false,
                grammar_constrained: false,
            },
            EngineKind::Tgi => Capabilities {
                streaming: true,
                embeddings: false,
                vision: false,
                function_calling: false,
                batch_inference: true,
                speculative_decoding: true,
                continuous_batching: true,
                quantization: true,
                lora_adapters: false,
                grammar_constrained: false,
            },
            EngineKind::OnnxRuntime => Capabilities {
                streaming: false,
                embeddings: true,
                vision: true,
                function_calling: false,
                batch_inference: true,
                speculative_decoding: false,
                continuous_batching: false,
                quantization: true,
                lora_adapters: false,
                grammar_constrained: false,
            },
        }
    }

    /// Returns true if this engine can satisfy all required capabilities.
    pub fn satisfies(&self, required: &Capabilities) -> bool {
        (!required.streaming || self.streaming)
            && (!required.embeddings || self.embeddings)
            && (!required.vision || self.vision)
            && (!required.function_calling || self.function_calling)
            && (!required.batch_inference || self.batch_inference)
            && (!required.speculative_decoding || self.speculative_decoding)
            && (!required.continuous_batching || self.continuous_batching)
            && (!required.quantization || self.quantization)
            && (!required.lora_adapters || self.lora_adapters)
            && (!required.grammar_constrained || self.grammar_constrained)
    }

    /// List all engine kinds that satisfy the required capabilities.
    pub fn matching_engines(required: &Capabilities) -> Vec<EngineKind> {
        let all = [
            EngineKind::LlamaCppServer,
            EngineKind::LlamaCppEmbedded,
            EngineKind::Ollama,
            EngineKind::Vllm,
            EngineKind::Sglang,
            EngineKind::LmStudio,
            EngineKind::Tgi,
            EngineKind::OnnxRuntime,
        ];
        all.into_iter()
            .filter(|k| Capabilities::for_engine(k).satisfies(required))
            .collect()
    }
}
