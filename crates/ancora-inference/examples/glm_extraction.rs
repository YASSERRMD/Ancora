/// GLM structured-output extraction snippet.
///
/// Demonstrates using GLM-5 with `response_format: json_object` to extract
/// structured data from unstructured text. GLM natively understands Chinese
/// and English; this example extracts company information from a news excerpt.
///
/// Run: `cargo run --example glm_extraction` (requires ZHIPU_API_KEY).
use ancora_inference::{
    openai::OpenAiClient,
    providers::glm::{build_glm_json_profile, is_json_object},
    types::{CompletionRequest, Message},
};
use std::sync::Arc;

fn build_extraction_request(text: &str) -> CompletionRequest {
    let system_prompt = "You are a data extraction assistant. Extract structured information \
        from the provided text and return it as a JSON object with these fields: \
        company_name (string), founded_year (integer or null), founder (string or null), \
        headquarters (string or null), products (array of strings).";
    CompletionRequest::simple(
        "glm-5",
        vec![
            Message::text("system", system_prompt),
            Message::text("user", text),
        ],
    )
}

fn main() {
    // Use the JSON profile so response_format is injected automatically
    let profile = Arc::new(build_glm_json_profile());
    let client = OpenAiClient::new(profile.clone());

    let news_excerpt = "Apple Inc., headquartered in Cupertino, California, \
        was co-founded by Steve Jobs, Steve Wozniak, and Ronald Wayne in 1976. \
        It designs and sells consumer electronics, software, and online services \
        including the iPhone, Mac, iPad, and Apple Watch.";

    let req = build_extraction_request(news_excerpt);

    println!("Provider: {}", "glm");
    println!("Model: {}", req.model_id);
    println!("JSON mode: active (response_format injected by profile transform)");
    println!();
    println!("Input text ({} chars):", news_excerpt.len());
    println!("  {}", &news_excerpt[..80]);
    println!("  ...");
    println!();

    // Demonstrate the is_json_object validator
    let sample_output = r#"{"company_name":"Apple Inc.","founded_year":1976,"founder":"Steve Jobs","headquarters":"Cupertino, CA","products":["iPhone","Mac","iPad","Apple Watch"]}"#;
    println!(
        "Sample expected output validates as JSON object: {}",
        is_json_object(sample_output)
    );
    println!();
    println!("// To send the request (requires ZHIPU_API_KEY):");
    println!("// let resp = client.complete(&req)?;");
    println!("// assert!(is_json_object(&resp.content));");
    println!("// let data: serde_json::Value = serde_json::from_str(&resp.content)?;");
    println!("// println!(\"Company: {{}}\", data[\"company_name\"]);");

    let _ = &client;
}
