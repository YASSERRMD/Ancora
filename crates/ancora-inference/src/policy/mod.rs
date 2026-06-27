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
}
