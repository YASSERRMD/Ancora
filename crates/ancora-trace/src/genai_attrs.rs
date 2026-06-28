/// GenAI semantic-convention attribute helpers.
///
/// Provides constants and builder helpers for the OpenTelemetry GenAI
/// semantic conventions as documented at:
/// https://opentelemetry.io/docs/specs/semconv/gen-ai/
///
/// All string keys follow the `gen_ai.*` namespace.

use crate::span::Span;

// --- Attribute key constants ---

pub const GEN_AI_SYSTEM: &str = "gen_ai.system";
pub const GEN_AI_REQUEST_MODEL: &str = "gen_ai.request.model";
pub const GEN_AI_RESPONSE_MODEL: &str = "gen_ai.response.model";
pub const GEN_AI_REQUEST_MAX_TOKENS: &str = "gen_ai.request.max_tokens";
pub const GEN_AI_REQUEST_TEMPERATURE: &str = "gen_ai.request.temperature";
pub const GEN_AI_USAGE_INPUT_TOKENS: &str = "gen_ai.usage.input_tokens";
pub const GEN_AI_USAGE_OUTPUT_TOKENS: &str = "gen_ai.usage.output_tokens";
pub const GEN_AI_PROMPT: &str = "gen_ai.prompt";
pub const GEN_AI_COMPLETION: &str = "gen_ai.completion";
pub const GEN_AI_OPERATION_NAME: &str = "gen_ai.operation.name";
pub const GEN_AI_RESPONSE_FINISH_REASONS: &str = "gen_ai.response.finish_reasons";

// --- Ancora-specific extensions ---

pub const ANCORA_TENANT_ID: &str = "ancora.tenant.id";
pub const ANCORA_RUN_ID: &str = "ancora.run.id";
pub const ANCORA_AGENT_ID: &str = "ancora.agent.id";
pub const ANCORA_TOOL_NAME: &str = "ancora.tool.name";
pub const ANCORA_COST_USD: &str = "ancora.cost.usd";
pub const ANCORA_RETRY_COUNT: &str = "ancora.retry.count";
pub const ANCORA_ERROR_KIND: &str = "ancora.error.kind";

/// GenAI model provider constants.
pub mod provider {
    pub const ANTHROPIC: &str = "anthropic";
    pub const OPENAI: &str = "openai";
    pub const GOOGLE: &str = "google";
    pub const MISTRAL: &str = "mistral";
    pub const AZURE: &str = "azure";
}

/// Attach GenAI request attributes to a span.
pub fn set_request_attrs(span: &mut Span, system: &str, model: &str, max_tokens: Option<i64>, temperature: Option<f64>) {
    span.set_attr_str(GEN_AI_SYSTEM, system);
    span.set_attr_str(GEN_AI_REQUEST_MODEL, model);
    if let Some(t) = max_tokens {
        span.set_attr_int(GEN_AI_REQUEST_MAX_TOKENS, t);
    }
    if let Some(t) = temperature {
        span.set_attr_float(GEN_AI_REQUEST_TEMPERATURE, t);
    }
}

/// Attach GenAI response / usage attributes to a span.
pub fn set_response_attrs(span: &mut Span, response_model: &str, input_tokens: i64, output_tokens: i64) {
    span.set_attr_str(GEN_AI_RESPONSE_MODEL, response_model);
    span.set_attr_int(GEN_AI_USAGE_INPUT_TOKENS, input_tokens);
    span.set_attr_int(GEN_AI_USAGE_OUTPUT_TOKENS, output_tokens);
}

/// Attach prompt and completion content to a span.
///
/// These attributes are subject to redaction; callers should apply a
/// `RedactPolicy` after calling this function if content is sensitive.
pub fn set_content_attrs(span: &mut Span, prompt: &str, completion: &str) {
    span.set_attr_str(GEN_AI_PROMPT, prompt);
    span.set_attr_str(GEN_AI_COMPLETION, completion);
}

/// Attach Ancora-specific tenant and run attributes.
pub fn set_run_attrs(span: &mut Span, tenant_id: &str, run_id: &str, agent_id: &str) {
    span.set_attr_str(ANCORA_TENANT_ID, tenant_id);
    span.set_attr_str(ANCORA_RUN_ID, run_id);
    span.set_attr_str(ANCORA_AGENT_ID, agent_id);
}

/// Attach cost attribution to a span.
pub fn set_cost_attr(span: &mut Span, cost_usd: f64) {
    span.set_attr_float(ANCORA_COST_USD, cost_usd);
}

/// Attach error information to a span.
pub fn set_error_attr(span: &mut Span, error_kind: &str, retry_count: i64) {
    span.set_attr_str(ANCORA_ERROR_KIND, error_kind);
    span.set_attr_int(ANCORA_RETRY_COUNT, retry_count);
}

/// Check whether a span has all required GenAI attributes.
pub fn has_required_genai_attrs(span: &Span) -> bool {
    span.attributes.contains_key(GEN_AI_SYSTEM)
        && span.attributes.contains_key(GEN_AI_REQUEST_MODEL)
}

/// Collect all `gen_ai.*` attribute keys present on a span.
pub fn genai_keys(span: &Span) -> Vec<&str> {
    span.attributes
        .keys()
        .filter(|k| k.starts_with("gen_ai."))
        .map(|k| k.as_str())
        .collect()
}

/// Collect all `ancora.*` attribute keys present on a span.
pub fn ancora_keys(span: &Span) -> Vec<&str> {
    span.attributes
        .keys()
        .filter(|k| k.starts_with("ancora."))
        .map(|k| k.as_str())
        .collect()
}

/// Retrieve a string attribute value.
pub fn get_str<'a>(span: &'a Span, key: &str) -> Option<&'a str> {
    span.attributes.get(key).and_then(|v| v.as_str())
}

/// Retrieve an integer attribute value.
pub fn get_int(span: &Span, key: &str) -> Option<i64> {
    span.attributes.get(key).and_then(|v| v.as_int())
}

/// Retrieve a float attribute value.
pub fn get_float(span: &Span, key: &str) -> Option<f64> {
    span.attributes.get(key).and_then(|v| v.as_float())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn request_attrs_set() {
        let mut s = Span::root("llm-call", 0);
        set_request_attrs(&mut s, provider::ANTHROPIC, "claude-3-5-sonnet", Some(4096), Some(0.7));
        assert_eq!(get_str(&s, GEN_AI_SYSTEM), Some(provider::ANTHROPIC));
        assert_eq!(get_str(&s, GEN_AI_REQUEST_MODEL), Some("claude-3-5-sonnet"));
        assert_eq!(get_int(&s, GEN_AI_REQUEST_MAX_TOKENS), Some(4096));
    }

    #[test]
    fn cost_attr_set() {
        let mut s = Span::root("llm-call", 0);
        set_cost_attr(&mut s, 0.0023);
        let cost = get_float(&s, ANCORA_COST_USD).unwrap();
        assert!((cost - 0.0023).abs() < 1e-9);
    }

    #[test]
    fn required_attrs_check() {
        let mut s = Span::root("llm-call", 0);
        assert!(!has_required_genai_attrs(&s));
        set_request_attrs(&mut s, provider::ANTHROPIC, "claude-3-haiku", None, None);
        assert!(has_required_genai_attrs(&s));
    }
}
