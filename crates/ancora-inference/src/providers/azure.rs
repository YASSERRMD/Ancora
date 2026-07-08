use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build an Azure OpenAI provider profile for a specific resource and deployment.
///
/// Azure-specific differences from OpenAI:
/// - base URL includes the resource name and deployment name
/// - auth uses `api-key` header (not `Authorization: Bearer`)
/// - path is `/chat/completions?api-version=<version>` (not `/v1/chat/completions`)
/// - the `model` field in the body is ignored by Azure (deployment encodes the model)
///
/// Reads the API key from `AZURE_OPENAI_API_KEY` at call time.
pub fn build_azure_profile(
    resource: impl AsRef<str>,
    deployment: impl AsRef<str>,
    api_version: impl AsRef<str>,
) -> ProviderProfile {
    let resource = resource.as_ref();
    let deployment = deployment.as_ref();
    let base_url = format!("https://{resource}.openai.azure.com/openai/deployments/{deployment}");
    let chat_path = format!("/chat/completions?api-version={}", api_version.as_ref());

    ProviderProfile::new(
        "azure-openai",
        base_url,
        AuthStrategy::HeaderKey {
            header: "api-key".to_owned(),
            env_var: "AZURE_OPENAI_API_KEY".to_owned(),
        },
    )
    .with_chat_path(chat_path)
    // Azure ignores the model field; remove it to keep the request clean.
    .with_request_transform(|body| {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("model");
        }
    })
    .add_model(
        ModelMeta::new(deployment, 128_000)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("default", deployment)
}

#[cfg(test)]
const FIXTURE: &str = r#"{"id":"chatcmpl-azure","choices":[{"message":{"role":"assistant","content":"Hello from Azure","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":6,"completion_tokens":3}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::OpenAiClient;
    use std::sync::Arc;

    fn client() -> OpenAiClient {
        OpenAiClient::new(Arc::new(build_azure_profile(
            "myresource",
            "gpt-4o-dep",
            "2024-02-01",
        )))
    }

    #[test]
    fn azure_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE, "gpt-4o-dep").unwrap();
        assert_eq!(resp.content, "Hello from Azure");
        assert_eq!(resp.tokens_in, 6);
        assert_eq!(resp.tokens_out, 3);
    }

    #[test]
    fn azure_deployment_name_in_base_url() {
        let profile = build_azure_profile("res", "dep1", "2024-02-01");
        assert!(profile.base_url.contains("dep1"));
        assert!(profile.base_url.contains("res.openai.azure.com"));
    }

    #[test]
    fn azure_api_version_in_chat_path() {
        let profile = build_azure_profile("res", "dep1", "2024-02-01");
        assert!(profile
            .chat_completions_path
            .contains("api-version=2024-02-01"));
    }

    #[test]
    fn azure_auth_via_api_key_header() {
        let profile = build_azure_profile("res", "dep1", "2024-02-01");
        match &profile.auth {
            AuthStrategy::HeaderKey { header, env_var } => {
                assert_eq!(header, "api-key");
                assert_eq!(env_var, "AZURE_OPENAI_API_KEY");
            }
            other => panic!("expected HeaderKey, got {other:?}"),
        }
    }

    #[test]
    fn azure_request_body_omits_model_field() {
        use crate::types::{CompletionRequest, Message};
        let p = Arc::new(build_azure_profile("res", "dep1", "2024-02-01"));
        let c = OpenAiClient::new(p);
        let req = CompletionRequest::simple("dep1", vec![Message::text("user", "hi")]);
        let body = c.build_request_body(&req, false).unwrap();
        assert!(
            body.get("model").is_none(),
            "model field should be removed for Azure"
        );
    }

    #[test]
    fn azure_completions_url_includes_api_version() {
        let profile = build_azure_profile("myres", "my-dep", "2024-05-01");
        let url = profile.completions_url(None);
        assert!(url.contains("api-version=2024-05-01"));
        assert!(url.contains("myres.openai.azure.com"));
        assert!(url.contains("my-dep"));
    }

    #[test]
    fn azure_deployment_name_routing() {
        // The deployment name is the model identifier for Azure.
        let profile = build_azure_profile("res", "my-deployment", "2024-02-01");
        let meta = profile.model_meta("my-deployment");
        assert!(meta.is_some(), "deployment should be registered as model");
    }

    #[test]
    fn azure_api_version_different_versions() {
        let p1 = build_azure_profile("r", "d", "2024-02-01");
        let p2 = build_azure_profile("r", "d", "2024-05-01-preview");
        assert!(p1.completions_url(None).contains("2024-02-01"));
        assert!(p2.completions_url(None).contains("2024-05-01-preview"));
    }

    #[test]
    fn azure_fixture_token_counts() {
        let resp = client().parse_response(FIXTURE, "gpt-4o-dep").unwrap();
        assert_eq!(resp.tokens_in, 6);
        assert_eq!(resp.tokens_out, 3);
    }

    #[test]
    fn azure_auth_header_name_is_api_key() {
        let profile = build_azure_profile("r", "d", "2024-02-01");
        match &profile.auth {
            AuthStrategy::HeaderKey { header, .. } => {
                assert_eq!(header, "api-key");
            }
            other => panic!("expected HeaderKey, got {other:?}"),
        }
    }

    #[test]
    fn azure_api_version_in_url_not_body() {
        use crate::types::{CompletionRequest, Message};
        let p = Arc::new(build_azure_profile("r", "d", "2024-02-01"));
        let c = OpenAiClient::new(p.clone());
        let req = CompletionRequest::simple("d", vec![Message::text("user", "hi")]);
        let body = c.build_request_body(&req, false).unwrap();
        // api-version must be in the URL, not in the JSON body
        assert!(
            body.get("api-version").is_none(),
            "api-version should not be in body"
        );
        assert!(p.completions_url(None).contains("api-version"));
    }
}
