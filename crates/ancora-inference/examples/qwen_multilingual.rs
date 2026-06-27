/// Qwen multilingual translation snippet.
///
/// Demonstrates using Qwen Plus (128k context) to translate text into multiple
/// languages in a single request. Qwen models natively support Chinese, Japanese,
/// Korean, Arabic, French, Spanish, and many other languages.
///
/// Run: `cargo run --example qwen_multilingual` (requires DASHSCOPE_API_KEY).
use ancora_inference::{
    openai::OpenAiClient,
    providers::qwen::build_qwen_profile,
    types::{CompletionRequest, Message},
};
use std::sync::Arc;

fn build_translation_request(source_text: &str, target_languages: &[&str]) -> CompletionRequest {
    let lang_list = target_languages.join(", ");
    let system_prompt = format!(
        "You are a professional translator. Translate the given text into: {lang_list}. \
         Return one translation per language, each labeled with the language name."
    );
    CompletionRequest::simple(
        "qwen-plus",
        vec![
            Message::text("system", &system_prompt),
            Message::text("user", source_text),
        ],
    )
}

fn build_region_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("sg", "Singapore (international default)"),
        ("eu", "Frankfurt (GDPR-compliant)"),
        ("us", "Virginia (US East)"),
        ("cn", "China domestic"),
    ]
}

fn main() {
    let profile = Arc::new(build_qwen_profile());
    let client = OpenAiClient::new(profile.clone());

    // Build a multilingual translation request
    let languages = ["Chinese (Simplified)", "Japanese", "French", "Arabic"];
    let req = build_translation_request(
        "Artificial intelligence is transforming the world.",
        &languages,
    );

    println!("Provider: {}", profile.name);
    println!("Model: {}", req.model_id);
    println!("Target languages: {}", languages.join(", "));
    println!();

    // Show all available regional endpoints
    println!("Available regional endpoints:");
    for (region, description) in build_region_info() {
        let url = profile.base_url_for_region(Some(region));
        println!("  {region}: {description}");
        println!("      -> {url}");
    }
    println!();

    println!("Messages: {}", req.messages.len());
    println!("System prompt length: {} chars", req.messages[0].content.len());
    println!();
    println!("// To send the request (requires DASHSCOPE_API_KEY):");
    println!("// let resp = client.complete(&req)?;");
    println!("// println!(\"{{}}\", resp.content);");

    // Verify the client is wired up correctly
    let _ = &client;
}
