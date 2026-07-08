/// Tests for engine capability detection.
use crate::capability::Capabilities;
use crate::model::EngineKind;

#[test]
fn vllm_supports_continuous_batching() {
    let caps = Capabilities::for_engine(&EngineKind::Vllm);
    assert!(caps.continuous_batching);
    assert!(caps.batch_inference);
    assert!(caps.streaming);
}

#[test]
fn llama_embedded_no_streaming() {
    let caps = Capabilities::for_engine(&EngineKind::LlamaCppEmbedded);
    assert!(!caps.streaming);
    assert!(caps.quantization);
    assert!(caps.embeddings);
}

#[test]
fn ollama_supports_vision() {
    let caps = Capabilities::for_engine(&EngineKind::Ollama);
    assert!(caps.vision);
    assert!(caps.function_calling);
}

#[test]
fn onnx_supports_batch_and_embeddings() {
    let caps = Capabilities::for_engine(&EngineKind::OnnxRuntime);
    assert!(caps.embeddings);
    assert!(caps.batch_inference);
    assert!(!caps.streaming);
}

#[test]
fn satisfies_returns_true_when_subset() {
    let engine_caps = Capabilities::for_engine(&EngineKind::Vllm);
    let required = Capabilities {
        streaming: true,
        quantization: true,
        ..Capabilities::default()
    };
    assert!(engine_caps.satisfies(&required));
}

#[test]
fn satisfies_returns_false_when_missing_capability() {
    let engine_caps = Capabilities::for_engine(&EngineKind::LlamaCppEmbedded);
    let required = Capabilities {
        streaming: true, // embedded doesn't support streaming
        ..Capabilities::default()
    };
    assert!(!engine_caps.satisfies(&required));
}

#[test]
fn matching_engines_finds_streaming_engines() {
    let required = Capabilities {
        streaming: true,
        ..Capabilities::default()
    };
    let matches = Capabilities::matching_engines(&required);
    // At least llama.cpp server, ollama, vllm, sglang, lm studio, tgi should match
    assert!(matches.contains(&EngineKind::LlamaCppServer));
    assert!(matches.contains(&EngineKind::Ollama));
    assert!(!matches.contains(&EngineKind::LlamaCppEmbedded));
    assert!(!matches.contains(&EngineKind::OnnxRuntime));
}

#[test]
fn matching_engines_with_lora_requirement() {
    let required = Capabilities {
        lora_adapters: true,
        ..Capabilities::default()
    };
    let matches = Capabilities::matching_engines(&required);
    assert!(matches.contains(&EngineKind::LlamaCppServer));
    assert!(matches.contains(&EngineKind::Vllm));
    assert!(matches.contains(&EngineKind::Sglang));
    assert!(!matches.contains(&EngineKind::Ollama));
}
