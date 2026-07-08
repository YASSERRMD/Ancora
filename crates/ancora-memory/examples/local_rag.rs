use ancora_memory::embedders::citation::{build_citations, filter_by_score, format_footnote_block};
use ancora_memory::embedders::context::ContextAssembler;
use ancora_memory::embedders::local::HashEmbedder;
use ancora_memory::embedders::pipeline::{PipelineConfig, RetrievalPipeline};
use ancora_memory::embedders::rerank::CosineReranker;
/// End-to-end local RAG (Retrieval-Augmented Generation) example.
///
/// Demonstrates the full offline retrieval pipeline:
///   1. Load and chunk documents.
///   2. Embed chunks with the local HashEmbedder.
///   3. Store and query the pipeline.
///   4. Rerank results with the CosineReranker.
///   5. Assemble a context window with the ContextAssembler.
///   6. Attach citation records.
///
/// No network calls, no API keys required.
use std::sync::Arc;

fn main() {
    println!("=== Local RAG Pipeline Demo ===\n");

    // 1. Prepare documents
    let docs_raw = vec![
        ("vector_search.md", include_str_or_fallback()),
        ("retrieval_basics.md", "Retrieval-Augmented Generation combines information retrieval with language models. \
         The retriever fetches relevant documents from a corpus. The generator synthesizes a response using those \
         documents as context. The quality of retrieval directly affects generation quality."),
        ("embeddings_guide.md", "Embedding models convert text into dense floating-point vectors. \
         Similar texts produce similar vectors. The cosine similarity between two vectors measures their semantic \
         relatedness. OpenAI, Cohere, and local models all produce embeddings in different dimensional spaces."),
    ];

    // 2. Load and display document counts
    let embedder: Arc<dyn ancora_memory::embedders::embedder::Embedder> =
        Arc::new(HashEmbedder::new(128));
    let config = PipelineConfig::new(20, 4, 5);
    let mut pipeline = RetrievalPipeline::new(Arc::clone(&embedder), config);

    for (source, content) in &docs_raw {
        let count = pipeline.ingest(source, content).unwrap();
        println!("Ingested '{source}': {count} chunks");
    }
    println!("Total passages in store: {}\n", pipeline.passage_count());

    // 3. Query
    let query = "how does retrieval work with language models";
    println!("Query: \"{query}\"\n");
    let results = pipeline.query(query).unwrap();

    // 4. Rerank with cosine similarity
    let q_emb = embedder.embed(query).unwrap();
    let passage_embs: Vec<_> = results
        .iter()
        .map(|r| embedder.embed(&r.text).unwrap())
        .collect();
    let reranker = CosineReranker::new(q_emb, passage_embs);
    let reranked_top = reranker.top_k(3);

    println!("Top-3 passages after reranking:");
    for (rank, (idx, score)) in reranked_top.iter().enumerate() {
        let text = &results[*idx].text;
        let preview = if text.len() > 80 { &text[..80] } else { text };
        println!(
            "  [{rank_n}] score={score:.4} -- {preview}...",
            rank_n = rank + 1
        );
    }
    println!();

    // 5. Assemble context
    let top_passages: Vec<&str> = reranked_top
        .iter()
        .map(|(idx, _)| results[*idx].text.as_str())
        .collect();
    let ctx = ContextAssembler::new(1000)
        .with_separator("\n---\n")
        .assemble(&top_passages);
    println!(
        "Context assembled: {} passages, ~{} tokens",
        ctx.passages_included, ctx.estimated_tokens
    );
    println!("Budget exhausted: {}", ctx.is_budget_exhausted());
    println!("\nContext preview (first 200 chars):");
    let preview = if ctx.text.len() > 200 {
        &ctx.text[..200]
    } else {
        &ctx.text
    };
    println!("{preview}...\n");

    // 6. Citations
    let citations_input: Vec<(&str, f32, &str)> = reranked_top
        .iter()
        .map(|(idx, score)| {
            (
                results[*idx].text.as_str(),
                *score,
                results[*idx].text.as_str(),
            )
        })
        .collect();
    // Map back to source; for demo just use placeholder.
    let cit_triples: Vec<(&str, f32, &str)> = citations_input
        .iter()
        .enumerate()
        .map(|(i, (_, score, text))| {
            let src = docs_raw
                .get(i % docs_raw.len())
                .map(|(s, _)| *s)
                .unwrap_or("unknown");
            (src, *score, *text)
        })
        .collect();
    let cits = build_citations(&cit_triples);
    let filtered = filter_by_score(cits, 0.0);
    println!("Citations:");
    println!("{}", format_footnote_block(&filtered));

    println!("\n=== Done ===");
}

fn include_str_or_fallback() -> &'static str {
    "Vector search is the process of finding similar vectors in a high-dimensional space. \
     It is commonly used in RAG systems to retrieve semantically similar documents. \
     Approximate Nearest Neighbor (ANN) algorithms such as HNSW and IVF enable sub-linear \
     query time even over millions of vectors. The quality of the underlying embedding model \
     determines the accuracy of semantic similarity."
}
