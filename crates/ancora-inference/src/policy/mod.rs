/// Data residency tags for provider endpoints.
///
/// Some providers route traffic through specific geographic regions. Callers
/// that have data residency obligations (e.g. GDPR, local data laws) can
/// use these tags to decide which providers are allowed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResidencyTag {
    /// Traffic and data may be processed in the EU.
    Eu,
    /// Traffic and data may be processed in the US.
    Us,
    /// Traffic and data may be processed in China (CN).
    Cn,
    /// Unknown or multiple regions.
    Unknown,
}

/// Return the residency tag(s) for a provider by name.
///
/// This is a best-effort mapping based on provider documentation.
pub fn residency_tags(provider_name: &str) -> Vec<ResidencyTag> {
    match provider_name {
        // DeepSeek's public API routes through CN infrastructure
        "deepseek" => vec![ResidencyTag::Cn],
        // Self-hosted DeepSeek: residency depends on where the host runs
        "deepseek-self-host" => vec![ResidencyTag::Unknown],
        // GLM (Zhipu AI) -- direct endpoint is CN-region
        "glm" => vec![ResidencyTag::Cn],
        "glm-self-host" => vec![ResidencyTag::Unknown],
        "glm-llamacpp" => vec![ResidencyTag::Unknown],
        // Qwen (DashScope) -- regional awareness
        // Default / Singapore international endpoint: non-CN, neutral
        "qwen" => vec![ResidencyTag::Us],
        // Explicit region-pinned variants (used when caller passes region label)
        "qwen-eu" => vec![ResidencyTag::Eu],
        "qwen-us" => vec![ResidencyTag::Us],
        "qwen-cn" => vec![ResidencyTag::Cn],
        // Self-hosted Qwen: residency depends on deployment
        "qwen-self-host" => vec![ResidencyTag::Unknown],
        // US-based providers
        "openai" | "groq" | "together" | "fireworks" | "anthropic" => vec![ResidencyTag::Us],
        // Azure: depends on deployment region, default US
        "azure" => vec![ResidencyTag::Us, ResidencyTag::Eu],
        // AWS Bedrock: depends on selected region
        "bedrock" => vec![ResidencyTag::Us, ResidencyTag::Eu],
        // EU-based providers
        "mistral" => vec![ResidencyTag::Eu],
        _ => vec![ResidencyTag::Unknown],
    }
}

/// Return `true` if the provider is allowed given a list of excluded regions.
pub fn is_allowed(provider_name: &str, excluded: &[ResidencyTag]) -> bool {
    let tags = residency_tags(provider_name);
    !tags.iter().any(|t| excluded.contains(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deepseek_direct_endpoint_tagged_cn() {
        let tags = residency_tags("deepseek");
        assert!(tags.contains(&ResidencyTag::Cn));
    }

    #[test]
    fn deepseek_self_host_tagged_unknown() {
        let tags = residency_tags("deepseek-self-host");
        assert!(tags.contains(&ResidencyTag::Unknown));
    }

    #[test]
    fn openai_tagged_us() {
        let tags = residency_tags("openai");
        assert!(tags.contains(&ResidencyTag::Us));
    }

    #[test]
    fn mistral_tagged_eu() {
        let tags = residency_tags("mistral");
        assert!(tags.contains(&ResidencyTag::Eu));
    }

    #[test]
    fn deepseek_direct_blocked_when_cn_excluded() {
        let excluded = vec![ResidencyTag::Cn];
        assert!(!is_allowed("deepseek", &excluded));
    }

    #[test]
    fn deepseek_self_host_allowed_when_cn_excluded() {
        // Self-host residency is unknown (user controls it), so not blocked
        let excluded = vec![ResidencyTag::Cn];
        assert!(is_allowed("deepseek-self-host", &excluded));
    }

    #[test]
    fn openai_allowed_when_cn_excluded() {
        let excluded = vec![ResidencyTag::Cn];
        assert!(is_allowed("openai", &excluded));
    }

    #[test]
    fn mistral_blocked_when_eu_excluded() {
        let excluded = vec![ResidencyTag::Eu];
        assert!(!is_allowed("mistral", &excluded));
    }

    #[test]
    fn deepseek_allowed_when_us_excluded() {
        // deepseek is CN, not US, so US-exclusion does not block it
        let excluded = vec![ResidencyTag::Us];
        assert!(is_allowed("deepseek", &excluded));
    }

    #[test]
    fn qwen_default_tagged_us() {
        let tags = residency_tags("qwen");
        assert!(tags.contains(&ResidencyTag::Us));
    }

    #[test]
    fn qwen_eu_variant_tagged_eu() {
        let tags = residency_tags("qwen-eu");
        assert!(tags.contains(&ResidencyTag::Eu));
    }

    #[test]
    fn qwen_cn_variant_tagged_cn() {
        let tags = residency_tags("qwen-cn");
        assert!(tags.contains(&ResidencyTag::Cn));
    }

    #[test]
    fn qwen_eu_allowed_under_eu_only() {
        // EU-only exclusion blocks CN and US; Frankfurt (qwen-eu) is allowed
        let excluded = vec![ResidencyTag::Cn, ResidencyTag::Us];
        assert!(is_allowed("qwen-eu", &excluded));
    }

    #[test]
    fn qwen_cn_blocked_under_eu_only_residency() {
        let excluded = vec![ResidencyTag::Cn];
        assert!(!is_allowed("qwen-cn", &excluded));
    }

    #[test]
    fn qwen_self_host_not_blocked_when_cn_excluded() {
        let excluded = vec![ResidencyTag::Cn];
        assert!(is_allowed("qwen-self-host", &excluded));
    }
}
