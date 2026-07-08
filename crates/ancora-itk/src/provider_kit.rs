//! Conformance kit for provider extensions.
//!
//! A provider must implement the [`Provider`] trait and pass all conformance
//! checks defined by [`ProviderKit`].

/// Minimal capability set that every provider extension must satisfy.
pub trait Provider {
    /// Return a human-readable name for the provider.
    fn name(&self) -> &str;

    /// Return the list of model identifiers this provider exposes.
    fn models(&self) -> Vec<String>;

    /// Complete a prompt and return the response text, or an error string.
    fn complete(&self, prompt: &str) -> Result<String, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`Provider`].
pub struct ProviderKit;

impl ProviderKit {
    pub fn new() -> Self {
        ProviderKit
    }

    /// Run all conformance checks and return the results.
    pub fn run<P: Provider>(&self, provider: &P) -> Vec<CheckResult> {
        vec![
            self.check_name(provider),
            self.check_models(provider),
            self.check_complete(provider),
        ]
    }

    fn check_name<P: Provider>(&self, provider: &P) -> CheckResult {
        let name = provider.name();
        if name.is_empty() {
            CheckResult {
                name: "provider_name_nonempty".into(),
                passed: false,
                message: "Provider name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "provider_name_nonempty".into(),
                passed: true,
                message: format!("Provider name: {name}"),
            }
        }
    }

    fn check_models<P: Provider>(&self, provider: &P) -> CheckResult {
        let models = provider.models();
        if models.is_empty() {
            CheckResult {
                name: "provider_has_models".into(),
                passed: false,
                message: "Provider must expose at least one model".into(),
            }
        } else {
            CheckResult {
                name: "provider_has_models".into(),
                passed: true,
                message: format!("{} model(s) advertised", models.len()),
            }
        }
    }

    fn check_complete<P: Provider>(&self, provider: &P) -> CheckResult {
        match provider.complete("ping") {
            Ok(response) if !response.is_empty() => CheckResult {
                name: "provider_complete_returns_text".into(),
                passed: true,
                message: format!("Received {} byte(s)", response.len()),
            },
            Ok(_) => CheckResult {
                name: "provider_complete_returns_text".into(),
                passed: false,
                message: "complete() returned an empty string".into(),
            },
            Err(e) => CheckResult {
                name: "provider_complete_returns_text".into(),
                passed: false,
                message: format!("complete() errored: {e}"),
            },
        }
    }
}

impl Default for ProviderKit {
    fn default() -> Self {
        Self::new()
    }
}
