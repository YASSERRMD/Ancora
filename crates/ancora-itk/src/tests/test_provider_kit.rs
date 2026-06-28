use crate::provider_kit::{Provider, ProviderKit};

struct EchoProvider;

impl Provider for EchoProvider {
    fn name(&self) -> &str {
        "echo-provider"
    }

    fn models(&self) -> Vec<String> {
        vec!["echo-v1".into()]
    }

    fn complete(&self, prompt: &str) -> Result<String, String> {
        Ok(format!("echo: {prompt}"))
    }
}

#[test]
fn provider_kit_passes_for_echo_provider() {
    let kit = ProviderKit::new();
    let results = kit.run(&EchoProvider);
    for r in &results {
        assert!(r.passed, "Check failed: {} - {}", r.name, r.message);
    }
    assert_eq!(results.len(), 3);
}

#[test]
fn provider_kit_check_names_are_unique() {
    let kit = ProviderKit::new();
    let results = kit.run(&EchoProvider);
    let names: std::collections::HashSet<_> = results.iter().map(|r| &r.name).collect();
    assert_eq!(names.len(), results.len());
}
