# Embeddings and Retrieval Pipeline

The `ancora-memory` crate ships a pluggable embeddings and RAG retrieval
pipeline in the `embedders` module.  Everything runs offline by default -- no
API keys or network calls are required unless you plug in a live provider.

---

## Architecture

```
Document
   |
   v
Loader (text / markdown / PDF stub)
   |
   v
Chunker (FixedSizeChunker or SemanticChunker)
   |
   v
Embedder (HashEmbedder | TfidfEmbedder | OpenAiEmbedder | CohereEmbedder | ...)
   |
   v
RetrievalPipeline (in-memory cosine ANN)
   |
   v
Reranker (CosineReranker | RRF | IdentityReranker)
   |
   v
ContextAssembler (token budget)
   |
   v
CitationRecord list
```

---

## Embedder trait

All providers implement `Embedder`:

```rust
pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> EmbedResult<Embedding>;
    fn embed_batch(&self, texts: &[&str]) -> EmbedResult<Vec<Embedding>>;
    fn model_name(&self) -> &str;
    fn dims(&self) -> usize;
}
```

Hold embedders behind `Arc<dyn Embedder>` so they can be shared across
pipeline stages.

---

## Built-in Providers

### Local (offline)

```rust
use ancora_memory::embedders::local::{HashEmbedder, TfidfEmbedder};

// Deterministic hash-based: no training data needed.
let hash = HashEmbedder::new(128);

// TF-IDF from a corpus:
let tfidf = TfidfEmbedder::fit(&["doc one text", "doc two text"], 500);
```

### OpenAI-compatible

```rust
use ancora_memory::embedders::openai::{OpenAiEmbedConfig, OpenAiEmbedder};

let cfg = OpenAiEmbedConfig::new(api_key, "text-embedding-3-small")
    .with_dimensions(256);
// Use the descriptor helpers to build request bodies; bring your own HTTP client.
```

### Cohere

```rust
use ancora_memory::embedders::cohere::{CohereConfig, embed_body, rerank_body};

let cfg = CohereConfig::new(api_key);
let body = embed_body(&cfg, &["passage"], cohere::input_type::SEARCH_DOCUMENT);
```

### Qwen / GLM

```rust
use ancora_memory::embedders::qwen_glm::{QwenEmbedConfig, qwen_embed_body};
use ancora_memory::embedders::qwen_glm::{GlmEmbedConfig, glm_embed_body};

let qwen = QwenEmbedConfig::new(dashscope_key);
let body = qwen_embed_body(&qwen, &["query text"]);
```

---

## Batch Embedding

`BatchEmbedder` splits large text batches and retries transient failures:

```rust
use ancora_memory::embedders::batch::{BatchEmbedder, BatchConfig};

let be = BatchEmbedder::new(
    HashEmbedder::new(128),
    BatchConfig::new(96).with_retries(3).skip_errors(),
);
let result = be.embed_all(&texts);
println!("success rate: {:.0}%", result.success_rate() * 100.0);
```

---

## Chunking

Two strategies are available:

```rust
use ancora_memory::embedders::chunker::{FixedSizeChunker, SemanticChunker};

// 200-word windows with 20-word overlap:
let fc = FixedSizeChunker::new(200, 20);
let chunks = fc.chunk("long document text...");

// Paragraph splits with 50-char minimum and 2 KB maximum:
let sc = SemanticChunker::paragraph().with_limits(50, 2000);
let chunks = sc.chunk("markdown document...");
```

---

## Document Loaders

```rust
use ancora_memory::embedders::loader::{load_text, load_markdown, split_markdown_sections, PdfLoader};

let doc = load_text("report.txt", &std::fs::read_to_string("report.txt").unwrap());

let md_sections = split_markdown_sections("guide.md", &md_content);

let pdf_loader = PdfLoader::new().with_max_pages(50);
let pages = pdf_loader.load_bytes("paper.pdf", &bytes);
```

---

## Retrieval Pipeline

```rust
use std::sync::Arc;
use ancora_memory::embedders::pipeline::{RetrievalPipeline, PipelineConfig};
use ancora_memory::embedders::local::HashEmbedder;
use ancora_memory::embedders::embedder::Embedder;

let embedder: Arc<dyn Embedder> = Arc::new(HashEmbedder::new(128));
let config = PipelineConfig::new(200, 20, 10);
let mut pipeline = RetrievalPipeline::new(embedder, config);

pipeline.ingest("doc.txt", "document content...").unwrap();
let results = pipeline.query("search query").unwrap();
let citations = pipeline.query_with_citations("search query").unwrap();
```

---

## Reranking

```rust
use ancora_memory::embedders::rerank::{CosineReranker, rrf_fuse};

// Cosine reranker (needs pre-computed embeddings):
let reranker = CosineReranker::new(query_emb, passage_embs);
let top_k = reranker.top_k(5);

// RRF fusion of dense and sparse ranked lists:
let fused = rrf_fuse(&dense_indices, &sparse_indices, 60.0, 10);
```

---

## Context Assembly

```rust
use ancora_memory::embedders::context::ContextAssembler;

let ctx = ContextAssembler::new(4096)
    .with_separator("\n---\n")
    .assemble(&passage_texts);

println!("Assembled {} passages (~{} tokens)", ctx.passages_included, ctx.estimated_tokens);
if ctx.is_budget_exhausted() {
    println!("Warning: {} passages dropped due to token budget", ctx.passages_dropped);
}
```

---

## Citations

```rust
use ancora_memory::embedders::citation::{build_citations, filter_by_score, format_footnote_block};

let cits = build_citations(&[("source.md", 0.9, "chunk text"), ...]);
let filtered = filter_by_score(cits, 0.5);
println!("{}", format_footnote_block(&filtered));
```

---

## Running the Example

```bash
cargo run -p ancora-memory --example local_rag
```

No environment variables or API keys needed.
