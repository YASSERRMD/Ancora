use std::collections::HashSet;

use crate::error::PolicyError;

/// Registry of endpoints permitted for egress.
pub struct EndpointRegistry {
    allowed: HashSet<String>,
    air_gapped: bool,
}

impl EndpointRegistry {
    pub fn new() -> Self {
        Self { allowed: HashSet::new(), air_gapped: false }
    }

    /// Enable air-gapped mode: reject all external egress regardless of allowed list.
    pub fn set_air_gapped(&mut self, value: bool) {
        self.air_gapped = value;
    }

    /// Allow egress to `endpoint` prefix (only effective when not air-gapped).
    pub fn allow(&mut self, endpoint: impl Into<String>) {
        self.allowed.insert(endpoint.into());
    }
}

impl Default for EndpointRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EndpointRegistry {
    /// Block model and tool calls to disallowed endpoints.
    pub fn check(&self, endpoint: &str) -> Result<(), PolicyError> {
        if self.air_gapped {
            return Err(PolicyError::ResidencyViolation(
                format!("air-gapped mode: all egress blocked (attempted '{endpoint}')")
            ));
        }
        if self.allowed.is_empty() {
            return Ok(());
        }
        let ok = self.allowed.iter().any(|a| endpoint.starts_with(a.as_str()));
        if !ok {
            return Err(PolicyError::ResidencyViolation(endpoint.to_owned()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn air_gapped_mode_rejects_all_external_egress() {
        let mut registry = EndpointRegistry::new();
        registry.set_air_gapped(true);
        registry.allow("https://eu.api.example.com");
        let err = registry.check("https://eu.api.example.com/v1").unwrap_err();
        assert!(matches!(err, PolicyError::ResidencyViolation(_)));
    }

    #[test]
    fn allowed_endpoint_passes_in_non_air_gapped_mode() {
        let mut registry = EndpointRegistry::new();
        registry.allow("https://eu.api.example.com");
        assert!(registry.check("https://eu.api.example.com/v1/chat").is_ok());
    }

    #[test]
    fn disallowed_endpoint_blocked_in_non_air_gapped_mode() {
        let mut registry = EndpointRegistry::new();
        registry.allow("https://eu.api.example.com");
        let err = registry.check("https://us.api.example.com/v1").unwrap_err();
        assert!(matches!(err, PolicyError::ResidencyViolation(_)));
    }
}
