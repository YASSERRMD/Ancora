use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the AWS Bedrock provider profile.
///
/// Bedrock requires AWS SigV4 request signing. The `BedrockSigV4` auth
/// strategy reads `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and
/// `AWS_SESSION_TOKEN` (optional) from the environment. The model ID is
/// embedded in the URL path, not the request body.
///
/// Supported model families: Anthropic Claude (via Bedrock), Amazon Titan,
/// Meta Llama 3, Mistral (Bedrock-hosted).
pub fn build_bedrock_profile() -> ProviderProfile {
    let region = std::env::var("AWS_REGION")
        .or_else(|_| std::env::var("AWS_DEFAULT_REGION"))
        .unwrap_or_else(|_| "us-east-1".to_owned());

    let base_url = format!("https://bedrock-runtime.{region}.amazonaws.com");

    ProviderProfile::new("bedrock", &base_url, AuthStrategy::None)
        .add_model(
            ModelMeta::new("anthropic.claude-3-5-sonnet-20241022-v2:0", 200_000)
                .with_pricing(3.0, 15.0)
                .with_tools()
                .with_vision()
                .with_streaming(),
        )
        .add_model(
            ModelMeta::new("anthropic.claude-3-haiku-20240307-v1:0", 200_000)
                .with_pricing(0.25, 1.25)
                .with_tools()
                .with_vision()
                .with_streaming(),
        )
        .add_model(
            ModelMeta::new("meta.llama3-70b-instruct-v1:0", 8_192)
                .with_pricing(0.99, 0.99)
                .with_streaming(),
        )
        .add_model(
            ModelMeta::new("mistral.mistral-large-2402-v1:0", 32_000)
                .with_pricing(4.0, 12.0)
                .with_tools()
                .with_streaming(),
        )
        .add_alias("claude-3-5-sonnet", "anthropic.claude-3-5-sonnet-20241022-v2:0")
        .add_alias("claude-haiku", "anthropic.claude-3-haiku-20240307-v1:0")
        .add_alias("llama3-70b", "meta.llama3-70b-instruct-v1:0")
        .add_alias("mistral-large", "mistral.mistral-large-2402-v1:0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bedrock_provider_name_is_bedrock() {
        let p = build_bedrock_profile();
        assert_eq!(p.name, "bedrock");
    }

    #[test]
    fn bedrock_base_url_contains_amazonaws() {
        let p = build_bedrock_profile();
        assert!(p.base_url.contains("amazonaws.com"));
    }

    #[test]
    fn bedrock_base_url_contains_bedrock_runtime() {
        let p = build_bedrock_profile();
        assert!(p.base_url.contains("bedrock-runtime"));
    }

    #[test]
    fn bedrock_claude_alias_resolves() {
        let p = build_bedrock_profile();
        assert_eq!(
            p.resolve_model_id("claude-3-5-sonnet"),
            "anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
    }

    #[test]
    fn bedrock_llama_alias_resolves() {
        let p = build_bedrock_profile();
        assert_eq!(p.resolve_model_id("llama3-70b"), "meta.llama3-70b-instruct-v1:0");
    }

    #[test]
    fn bedrock_claude_has_vision() {
        let p = build_bedrock_profile();
        let meta = p.model_meta("anthropic.claude-3-5-sonnet-20241022-v2:0").unwrap();
        assert!(meta.capabilities.vision);
    }

    #[test]
    fn bedrock_claude_has_tools() {
        let p = build_bedrock_profile();
        let meta = p.model_meta("anthropic.claude-3-5-sonnet-20241022-v2:0").unwrap();
        assert!(meta.capabilities.tools);
    }

    #[test]
    fn bedrock_llama_has_no_tools() {
        let p = build_bedrock_profile();
        let meta = p.model_meta("meta.llama3-70b-instruct-v1:0").unwrap();
        assert!(!meta.capabilities.tools);
    }

    #[test]
    fn bedrock_mistral_has_tools() {
        let p = build_bedrock_profile();
        let meta = p.model_meta("mistral.mistral-large-2402-v1:0").unwrap();
        assert!(meta.capabilities.tools);
    }

    #[test]
    fn bedrock_claude_haiku_pricing_cheaper_than_sonnet() {
        let p = build_bedrock_profile();
        let sonnet = p.model_meta("anthropic.claude-3-5-sonnet-20241022-v2:0").unwrap();
        let haiku = p.model_meta("anthropic.claude-3-haiku-20240307-v1:0").unwrap();
        let sp = sonnet.pricing.as_ref().unwrap();
        let hp = haiku.pricing.as_ref().unwrap();
        assert!(hp.input_per_million < sp.input_per_million);
    }
}
