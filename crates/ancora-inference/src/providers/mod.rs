pub mod anthropic;
pub mod azure;
pub mod bedrock;
pub mod cohere;
pub mod deepseek;
pub mod doubao;
pub mod fireworks;
pub mod ernie;
pub mod gateway;
pub mod gemini;
pub mod hunyuan;
pub mod glm;
pub mod groq;
pub mod kimi;
pub mod litellm;
pub mod mimo;
pub mod minimax;
pub mod mistral;
pub mod openai;
pub mod openrouter;
pub mod portkey;
pub mod qwen;
pub mod stepfun;
pub mod throughput;
pub mod together;
pub mod usage;
pub mod vercelai;

/// Normalize an HTTP error for any of the five new Chinese-lab providers.
///
/// Dispatches to the provider-specific `normalize_error` when one is available,
/// otherwise falls through to the generic `InferenceError::from_http`.
pub fn normalize_chinese_lab_error(
    provider_name: &str,
    status: u16,
    body: &str,
) -> crate::error::InferenceError {
    match provider_name {
        "stepfun" => stepfun::normalize_error(status, body),
        "ernie" => ernie::normalize_error(status, body),
        "hunyuan" => hunyuan::normalize_error(status, body),
        "doubao" | "doubao-self-host" => doubao::normalize_error(status, body),
        "mimo" | "mimo-local" => mimo::normalize_error(status, body),
        _ => crate::error::InferenceError::from_http(status, body, None),
    }
}
