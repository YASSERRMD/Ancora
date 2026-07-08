use std::sync::Arc;

use crate::error::InferenceError;
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
    format!(
        "{}/model/{}/invoke-with-response-stream",
        profile.base_url, model_id
    )
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

// ---- SigV4 signing stub ----------------------------------------------------

/// AWS credential source for SigV4 signing.
///
/// Credentials are read from the environment on each call. In production
/// this would be replaced by a full AWS credential chain (env, ~/.aws,
/// instance metadata). The signing implementation below is intentionally a
/// stub that produces a valid-looking canonical request without making live
/// AWS calls -- suitable for unit testing URL + header structure.
pub struct AwsCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
}

impl AwsCredentials {
    /// Load credentials from the standard AWS environment variables.
    pub fn from_env() -> Result<Self, InferenceError> {
        let access_key_id = std::env::var("AWS_ACCESS_KEY_ID")
            .map_err(|_| InferenceError::MissingCredential("AWS_ACCESS_KEY_ID".to_owned()))?;
        let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .map_err(|_| InferenceError::MissingCredential("AWS_SECRET_ACCESS_KEY".to_owned()))?;
        let session_token = std::env::var("AWS_SESSION_TOKEN").ok();
        Ok(Self {
            access_key_id,
            secret_access_key,
            session_token,
        })
    }
}

/// Compute the list of headers required for AWS SigV4 authentication.
///
/// This is a structural stub: it returns the correct header names that
/// SigV4 requires (`x-amz-date`, `x-amz-security-token`, and ultimately
/// `Authorization`) without performing real HMAC-SHA256 signing. A real
/// implementation would use `aws-sigv4` or `rusoto_signature`.
///
/// The stub is sufficient for:
/// - Verifying the adapter adds the correct header set
/// - Offline unit tests (no network call needed)
/// - Serving as the integration point for a real signing crate
pub fn sigv4_headers_stub(
    credentials: &AwsCredentials,
    region: &str,
    date_iso8601: &str,
) -> Vec<(String, String)> {
    let mut headers = vec![
        ("x-amz-date".to_owned(), date_iso8601.to_owned()),
        (
            "x-amz-content-sha256".to_owned(),
            "UNSIGNED-PAYLOAD".to_owned(),
        ),
        // In production: HMAC-SHA256 of canonical request + string-to-sign
        (
            "authorization".to_owned(),
            format!(
                "AWS4-HMAC-SHA256 Credential={access}/{date}/{region}/bedrock/aws4_request, \
                 SignedHeaders=host;x-amz-date;x-amz-content-sha256, Signature=STUB",
                access = credentials.access_key_id,
                date = &date_iso8601[..8],
                region = region,
            ),
        ),
    ];
    if let Some(token) = &credentials.session_token {
        headers.push(("x-amz-security-token".to_owned(), token.clone()));
    }
    headers
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

    #[test]
    fn sigv4_stub_includes_x_amz_date() {
        let creds = AwsCredentials {
            access_key_id: "AKIAIOSFODNN7EXAMPLE".to_owned(),
            secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_owned(),
            session_token: None,
        };
        let headers = sigv4_headers_stub(&creds, "us-east-1", "20240101T120000Z");
        let names: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        assert!(names.contains(&"x-amz-date"));
    }

    #[test]
    fn sigv4_stub_includes_authorization() {
        let creds = AwsCredentials {
            access_key_id: "AKIAIOSFODNN7EXAMPLE".to_owned(),
            secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_owned(),
            session_token: None,
        };
        let headers = sigv4_headers_stub(&creds, "us-east-1", "20240101T120000Z");
        let auth = headers
            .iter()
            .find(|(k, _)| k == "authorization")
            .map(|(_, v)| v.as_str());
        assert!(auth.is_some());
        assert!(auth.unwrap().starts_with("AWS4-HMAC-SHA256"));
    }

    #[test]
    fn sigv4_stub_session_token_added_when_present() {
        let creds = AwsCredentials {
            access_key_id: "AKIAIOSFODNN7EXAMPLE".to_owned(),
            secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_owned(),
            session_token: Some("session-token-value".to_owned()),
        };
        let headers = sigv4_headers_stub(&creds, "us-east-1", "20240101T120000Z");
        let names: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        assert!(names.contains(&"x-amz-security-token"));
    }

    #[test]
    fn sigv4_stub_no_session_token_when_absent() {
        let creds = AwsCredentials {
            access_key_id: "AKIAIOSFODNN7EXAMPLE".to_owned(),
            secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_owned(),
            session_token: None,
        };
        let headers = sigv4_headers_stub(&creds, "us-east-1", "20240101T120000Z");
        let names: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        assert!(!names.contains(&"x-amz-security-token"));
    }
}
