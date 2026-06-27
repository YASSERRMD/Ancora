pub mod auth;
pub mod meta;
pub mod registry;
pub mod transform;

pub use auth::AuthStrategy;
pub use meta::{CapabilityFlags, ModelMeta, PricingMeta};
pub use registry::ProviderRegistry;
pub use transform::{RequestTransformChain, ResponseTransformChain};

use std::collections::HashMap;

/// Configuration for a single provider.
///
/// A new provider is added by registering a `ProviderProfile` in a `ProviderRegistry` --
/// no new client type is required. See `docs/guides/adding-a-provider.md`.
pub struct ProviderProfile {
    pub name: String,
    pub base_url: String,
    pub auth: AuthStrategy,
    /// Path appended to the base URL to reach the chat-completions endpoint.
    ///
    /// Defaults to `"/v1/chat/completions"` (OpenAI standard). Azure uses
    /// `"/chat/completions"` with an api-version suffix.
    pub chat_completions_path: String,
    /// Fixed headers added to every HTTP request (e.g. OpenRouter app-attribution).
    pub extra_headers: HashMap<String, String>,
    /// Per-model metadata keyed by canonical model-id.
    pub model_catalog: HashMap<String, ModelMeta>,
    /// Short-name aliases pointing at canonical model-ids.
    pub model_aliases: HashMap<String, String>,
    /// Base-URL overrides keyed by region label (e.g. `"eu"`, `"us"`, `"cn"`).
    pub regional_urls: HashMap<String, String>,
    pub request_transforms: RequestTransformChain,
    pub response_transforms: ResponseTransformChain,
}

impl ProviderProfile {
    pub fn new(
        name: impl Into<String>,
        base_url: impl Into<String>,
        auth: AuthStrategy,
    ) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into(),
            auth,
            chat_completions_path: "/v1/chat/completions".to_owned(),
            extra_headers: HashMap::new(),
            model_catalog: HashMap::new(),
            model_aliases: HashMap::new(),
            regional_urls: HashMap::new(),
            request_transforms: RequestTransformChain::default(),
            response_transforms: ResponseTransformChain::default(),
        }
    }

    /// Override the chat-completions endpoint path (e.g. for Azure).
    pub fn with_chat_path(mut self, path: impl Into<String>) -> Self {
        self.chat_completions_path = path.into();
        self
    }

    /// Add a fixed HTTP header sent with every request (e.g. for app-attribution).
    pub fn with_extra_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_headers.insert(name.into(), value.into());
        self
    }

    /// Register model metadata into the catalog.
    pub fn add_model(mut self, meta: ModelMeta) -> Self {
        self.model_catalog.insert(meta.model_id.clone(), meta);
        self
    }

    /// Register a short-name alias for a canonical model-id.
    pub fn add_alias(
        mut self,
        alias: impl Into<String>,
        canonical: impl Into<String>,
    ) -> Self {
        self.model_aliases.insert(alias.into(), canonical.into());
        self
    }

    /// Add a regional base-URL override.
    pub fn add_region(
        mut self,
        region: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        self.regional_urls.insert(region.into(), url.into());
        self
    }

    /// Push a request-body transform applied before every HTTP call.
    pub fn with_request_transform(
        mut self,
        f: impl Fn(&mut serde_json::Value) + Send + Sync + 'static,
    ) -> Self {
        self.request_transforms.push(f);
        self
    }

    /// Push a response-body transform applied after every HTTP response.
    pub fn with_response_transform(
        mut self,
        f: impl Fn(&mut serde_json::Value) + Send + Sync + 'static,
    ) -> Self {
        self.response_transforms.push(f);
        self
    }

    /// Resolve a user-supplied model id through the alias map.
    pub fn resolve_model_id<'a>(&'a self, id: &'a str) -> &'a str {
        self.model_aliases.get(id).map(|s| s.as_str()).unwrap_or(id)
    }

    /// Look up metadata for a model, resolving aliases first.
    pub fn model_meta(&self, id: &str) -> Option<&ModelMeta> {
        let canonical = self.resolve_model_id(id);
        self.model_catalog.get(canonical)
    }

    /// Effective base URL for a given optional region label.
    pub fn base_url_for_region(&self, region: Option<&str>) -> &str {
        region
            .and_then(|r| self.regional_urls.get(r))
            .map(|s| s.as_str())
            .unwrap_or(&self.base_url)
    }

    /// Full chat-completions URL for a given optional region.
    pub fn completions_url(&self, region: Option<&str>) -> String {
        format!(
            "{}{}",
            self.base_url_for_region(region).trim_end_matches('/'),
            self.chat_completions_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_profile() -> ProviderProfile {
        ProviderProfile::new("acme", "https://api.acme.test", AuthStrategy::None)
            .add_model(ModelMeta::new("acme-large", 128_000).with_pricing(5.0, 15.0))
            .add_alias("large", "acme-large")
            .add_region("eu", "https://eu.api.acme.test")
    }

    #[test]
    fn resolve_alias_to_canonical() {
        let p = build_profile();
        assert_eq!(p.resolve_model_id("large"), "acme-large");
    }

    #[test]
    fn unresolved_alias_returns_original() {
        let p = build_profile();
        assert_eq!(p.resolve_model_id("acme-large"), "acme-large");
    }

    #[test]
    fn model_meta_via_alias() {
        let p = build_profile();
        let meta = p.model_meta("large").expect("meta not found via alias");
        assert_eq!(meta.model_id, "acme-large");
    }

    #[test]
    fn base_url_for_known_region() {
        let p = build_profile();
        assert_eq!(p.base_url_for_region(Some("eu")), "https://eu.api.acme.test");
    }

    #[test]
    fn base_url_falls_back_for_unknown_region() {
        let p = build_profile();
        assert_eq!(p.base_url_for_region(Some("us")), "https://api.acme.test");
        assert_eq!(p.base_url_for_region(None), "https://api.acme.test");
    }

    #[test]
    fn request_transform_applied_via_builder() {
        let p = ProviderProfile::new("t", "http://x", AuthStrategy::None)
            .with_request_transform(|v| v["injected"] = serde_json::json!(42));
        let mut body = serde_json::json!({});
        p.request_transforms.apply(&mut body);
        assert_eq!(body["injected"], serde_json::json!(42));
    }

    #[test]
    fn completions_url_default_path() {
        let p = ProviderProfile::new("p", "https://api.test", AuthStrategy::None);
        assert_eq!(p.completions_url(None), "https://api.test/v1/chat/completions");
    }

    #[test]
    fn completions_url_custom_path() {
        let p = ProviderProfile::new("az", "https://my.openai.azure.com/openai/deployments/dep", AuthStrategy::None)
            .with_chat_path("/chat/completions?api-version=2024-02-01");
        assert_eq!(
            p.completions_url(None),
            "https://my.openai.azure.com/openai/deployments/dep/chat/completions?api-version=2024-02-01"
        );
    }

    #[test]
    fn extra_header_stored_in_profile() {
        let p = ProviderProfile::new("or", "https://openrouter.ai/api", AuthStrategy::None)
            .with_extra_header("HTTP-Referer", "https://myapp.test")
            .with_extra_header("X-Title", "MyApp");
        assert_eq!(p.extra_headers.get("HTTP-Referer").map(|s| s.as_str()), Some("https://myapp.test"));
        assert_eq!(p.extra_headers.get("X-Title").map(|s| s.as_str()), Some("MyApp"));
    }
}
