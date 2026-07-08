pub mod capability;
pub mod health;
pub mod llama_embedded;
pub mod llama_server;
pub mod lmstudio;
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
pub mod ollama;
pub mod onnx;
pub mod runtime;
pub mod sglang;
pub mod tgi;
pub mod vllm;

#[cfg(test)]
mod tests;
