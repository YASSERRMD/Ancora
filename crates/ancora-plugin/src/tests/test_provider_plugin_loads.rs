//! Tests: a sample provider plugin loads and responds correctly.

use crate::manifest::{ManifestBuilder, PluginKind, SemVer};
use crate::provider_ext::{EchoProvider, Message, GenerateRequest, ProviderPlugin, Role};

fn build_provider_manifest() -> crate::manifest::PluginManifest {
    ManifestBuilder::new()
        .id("echo-provider")
        .name("Echo Provider")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0))
        .kind(PluginKind::Provider)
        .build()
        .unwrap()
}

#[test]
fn echo_provider_lists_models() {
    let p = EchoProvider::new("echo-provider");
    let models = p.list_models();
    assert!(!models.is_empty());
    assert!(models.contains(&"echo-1".to_string()));
}

#[test]
fn echo_provider_generates_response() {
    let p = EchoProvider::new("echo-provider");
    let req = GenerateRequest {
        model: "echo-1".to_string(),
        messages: vec![Message { role: Role::User, content: "hello".to_string() }],
        max_tokens: None,
        temperature: None,
        stop_sequences: vec![],
    };
    let resp = p.generate(req).unwrap();
    assert!(resp.content.contains("hello"));
    assert!(!resp.truncated);
}

#[test]
fn provider_manifest_is_valid() {
    let m = build_provider_manifest();
    assert_eq!(m.id, "echo-provider");
    assert_eq!(m.kind, PluginKind::Provider);
}

#[test]
fn provider_id_matches_manifest() {
    let p = EchoProvider::new("echo-provider");
    let m = build_provider_manifest();
    assert_eq!(p.provider_id(), m.id.as_str());
}
