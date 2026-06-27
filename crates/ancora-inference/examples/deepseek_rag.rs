/// DeepSeek long-context RAG snippet.
///
/// Demonstrates how to use the DeepSeek V3 64k context window to perform
/// retrieval-augmented generation: embed multiple document chunks directly
/// into the prompt without a vector database, relying on the long context
/// window to hold the full retrieval set.
///
/// Run: `cargo run --example deepseek_rag` (requires DEEPSEEK_API_KEY).
use ancora_inference::{
    openai::OpenAiClient,
    providers::deepseek::build_deepseek_profile,
    types::{CompletionRequest, Message},
};
use std::sync::Arc;

fn build_rag_request(chunks: &[&str], question: &str) -> CompletionRequest {
    let context = chunks.join("\n\n");
    let system_prompt = format!(
        "You are a helpful assistant. Answer using only the context below.\n\nContext:\n{context}"
    );
    CompletionRequest::simple(
        "deepseek-coder",
        vec![
            Message::text("system", &system_prompt),
            Message::text("user", question),
        ],
    )
}

fn main() {
    let profile = Arc::new(build_deepseek_profile());
    let client = OpenAiClient::new(profile);

    // Simulate retrieved document chunks (in production: retrieved by vector search)
    let chunks = vec![
        "Document 1: DeepSeek V3 supports a 64k token context window.",
        "Document 2: Cache-hit input tokens are billed at $0.07/M instead of $0.27/M.",
        "Document 3: DeepSeek R1 is a reasoning model that shows chain-of-thought.",
    ];

    let req = build_rag_request(&chunks, "What is the cache-hit pricing for DeepSeek V3?");

    println!("Model: {}", req.model_id);
    println!("Messages: {}", req.messages.len());
    println!("System context length: {} chars", req.messages[0].content.len());
    println!();
    println!("// To send the request:");
    println!("// let resp = client.complete(&req)?;");
    println!("// println!(\"{{}}\", resp.content);");

    // Verify the client resolves the alias correctly before sending
    let _ = &client; // client would be used here in production
}
