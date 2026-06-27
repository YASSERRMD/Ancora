use std::sync::Arc;

use crate::provider::ProviderProfile;

/// Build the Bedrock invocation URL for a given model ID.
///
/// Bedrock embeds the model ID directly in the URL path:
///   `https://bedrock-runtime.<region>.amazonaws.com/model/<model-id>/invoke`
/// For streaming: `…/model/<model-id>/invoke-with-response-stream`
pub fn invocation_url(profile: &ProviderProfile, model_id: &str) -> String {
    format!("{}/model/{}/invoke", profile.base_url, model_id)
}

/// Build the Bedrock streaming invocation URL for a given model ID.
pub fn stream_url(profile: &ProviderProfile, model_id: &str) -> String {
    format!("{}/model/{}/invoke-with-response-stream", profile.base_url, model_id)
}

/// Extract the AWS region from the Bedrock base URL.
///
/// Base URL format: `https://bedrock-runtime.<region>.amazonaws.com`
pub fn region_from_url(base_url: &str) -> Option<&str> {
    // Strip prefix up to "bedrock-runtime."
    let after_prefix = base_url.split("bedrock-runtime.").nth(1)?;
    // Region is everything before ".amazonaws.com"
    after_prefix.split(".amazonaws.com").next()
}

/// The Bedrock adapter is a stub.
///
/// Full SigV4 signing is implemented in the next commit. This type holds
/// the profile reference for URL construction and will carry signing
/// credentials once the signing logic is wired up.
pub struct BedrockClient {
    pub(crate) profile: Arc<ProviderProfile>,
}

impl BedrockClient {
    pub fn new(profile: Arc<ProviderProfile>) -> Self {
        Self { profile }
    }

    /// Build the invocation URL for the given (already-resolved) model ID.
    pub fn url_for(&self, model_id: &str) -> String {
        invocation_url(&self.profile, model_id)
    }

    /// Build the streaming invocation URL for the given model ID.
    pub fn stream_url_for(&self, model_id: &str) -> String {
        stream_url(&self.profile, model_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::bedrock::build_bedrock_profile;

    fn client() -> BedrockClient {
        BedrockClient::new(Arc::new(build_bedrock_profile()))
    }

    #[test]
    fn bedrock_url_contains_model_id() {
        let c = client();
        let url = c.url_for("anthropic.claude-3-5-sonnet-20241022-v2:0");
        assert!(url.contains("anthropic.claude-3-5-sonnet-20241022-v2:0"));
    }

    #[test]
    fn bedrock_url_ends_with_invoke() {
        let c = client();
        let url = c.url_for("anthropic.claude-3-5-sonnet-20241022-v2:0");
        assert!(url.ends_with("/invoke"));
    }

    #[test]
    fn bedrock_stream_url_ends_with_invoke_with_response_stream() {
        let c = client();
        let url = c.stream_url_for("meta.llama3-70b-instruct-v1:0");
        assert!(url.ends_with("/invoke-with-response-stream"));
    }

    #[test]
    fn bedrock_url_contains_bedrock_runtime() {
        let c = client();
        let url = c.url_for("anthropic.claude-3-5-sonnet-20241022-v2:0");
        assert!(url.contains("bedrock-runtime"));
    }

    #[test]
    fn region_from_url_extracts_region() {
        let url = "https://bedrock-runtime.us-east-1.amazonaws.com";
        assert_eq!(region_from_url(url), Some("us-east-1"));
    }

    #[test]
    fn region_from_url_extracts_eu_region() {
        let url = "https://bedrock-runtime.eu-west-1.amazonaws.com";
        assert_eq!(region_from_url(url), Some("eu-west-1"));
    }
}
