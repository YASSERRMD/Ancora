use std::collections::HashMap;
use super::ProviderProfile;

/// Registry of named provider profiles.
///
/// Add a provider once; look it up by name anywhere a client is constructed.
#[derive(Default)]
pub struct ProviderRegistry {
    profiles: HashMap<String, ProviderProfile>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider profile; replaces any existing entry with the same name.
    pub fn register(&mut self, profile: ProviderProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Look up a profile by name.
    pub fn get(&self, name: &str) -> Option<&ProviderProfile> {
        self.profiles.get(name)
    }

    /// Iterate over all registered provider names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.profiles.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{AuthStrategy, ProviderProfile};

    fn fake_profile(name: &str) -> ProviderProfile {
        ProviderProfile::new(name, "http://localhost", AuthStrategy::None)
    }

    #[test]
    fn registry_resolves_profile_by_name() {
        let mut reg = ProviderRegistry::new();
        reg.register(fake_profile("acme"));
        let profile = reg.get("acme").expect("profile not found");
        assert_eq!(profile.name, "acme");
    }

    #[test]
    fn registry_returns_none_for_unknown_provider() {
        let reg = ProviderRegistry::new();
        assert!(reg.get("missing").is_none());
    }

    #[test]
    fn registry_replaces_on_duplicate_name() {
        let mut reg = ProviderRegistry::new();
        reg.register(fake_profile("p"));
        reg.register(fake_profile("p"));
        assert_eq!(reg.len(), 1);
    }
}
