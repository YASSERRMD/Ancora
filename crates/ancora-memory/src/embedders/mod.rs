pub mod batch;
pub mod chunker;
pub mod chunker_tests;
pub mod citation;
pub mod cohere;
pub mod context;
pub mod context_tests;
/// Pluggable embedding providers and retrieval pipeline for ancora-memory.
///
/// Modules:
/// - `embedder`       -- core Embedder trait + common types
/// - `openai`         -- OpenAI-compatible embedding API helpers
/// - `local`          -- offline/deterministic local embedder
/// - `cohere`         -- Cohere embed + rerank API helpers
/// - `qwen_glm`       -- Qwen/GLM embedding endpoint helpers
/// - `batch`          -- batch embedding with backpressure
/// - `chunker`        -- fixed-size and semantic chunkers
/// - `loader`         -- text, markdown, and PDF document loaders
/// - `pipeline`       -- full retrieval pipeline (embed, chunk, store, query)
/// - `rerank`         -- optional rerank stage
/// - `context`        -- context assembly with token budget
/// - `citation`       -- citation metadata passthrough
pub mod embedder;
pub mod embedder_tests;
pub mod loader;
pub mod loader_tests;
pub mod local;
pub mod openai;
pub mod pipeline;
pub mod pipeline_tests;
pub mod qwen_glm;
pub mod rerank;
pub mod rerank_tests;
