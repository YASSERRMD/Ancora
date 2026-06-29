/// ancora-localeng: deep integration with local inference engines.
///
/// Supported engines:
/// - llama.cpp server (HTTP mode)
/// - llama.cpp embedded (in-process)
/// - Ollama
/// - vLLM
/// - SGLang
/// - LM Studio
/// - Text Generation Inference (TGI)
/// - ONNX Runtime
///
/// All engines are backed by a pluggable transport trait so tests
/// run fully offline against mocks.

pub mod model;
pub mod llama_server;
pub mod llama_embedded;
pub mod ollama;
pub mod vllm;
pub mod sglang;
pub mod lmstudio;
pub mod tgi;
pub mod onnx;
pub mod capability;
pub mod health;
pub mod runtime;

#[cfg(test)]
mod tests;
