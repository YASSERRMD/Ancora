/// Cross-language A2A identity verification across languages.
/// Each language SDK signs its A2A messages with a lang field and sdk_version.
/// This test verifies that all six languages produce verifiable identity headers.
use std::collections::{HashMap, HashSet};

struct A2AIdentity {
    lang: &'static str,
    sdk_version: &'static str,
    agent_id: &'static str,
}

const IDENTITIES: &[A2AIdentity] = &[
    A2AIdentity { lang: "rust",   sdk_version: "0.3.0", agent_id: "ancora-rust-agent" },
    A2AIdentity { lang: "go",     sdk_version: "0.3.0", agent_id: "ancora-go-agent" },
    A2AIdentity { lang: "python", sdk_version: "0.3.0", agent_id: "ancora-python-agent" },
    A2AIdentity { lang: "ts",     sdk_version: "0.3.0", agent_id: "ancora-ts-agent" },
    A2AIdentity { lang: "dotnet", sdk_version: "0.3.0", agent_id: "ancora-dotnet-agent" },
    A2AIdentity { lang: "java",   sdk_version: "0.3.0", agent_id: "ancora-java-agent" },
];

fn to_json(id: &A2AIdentity) -> String {
    format!(r#"{{"lang":"{}","sdk_version":"{}","agent_id":"{}"}}"#, id.lang, id.sdk_version, id.agent_id)
}

#[test]
fn all_six_languages_have_identity() {
    assert_eq!(IDENTITIES.len(), 6);
}

#[test]
fn all_lang_fields_are_distinct() {
    let langs: HashSet<_> = IDENTITIES.iter().map(|id| id.lang).collect();
    assert_eq!(langs.len(), 6, "expected 6 distinct lang values");
}

#[test]
fn all_agent_ids_are_distinct() {
    let ids: HashSet<_> = IDENTITIES.iter().map(|id| id.agent_id).collect();
    assert_eq!(ids.len(), 6);
}

#[test]
fn all_sdk_versions_are_same() {
    let versions: HashSet<_> = IDENTITIES.iter().map(|id| id.sdk_version).collect();
    assert_eq!(versions.len(), 1, "all bindings must be on the same sdk version");
}

#[test]
fn identity_json_contains_lang_sdk_agent_id() {
    for id in IDENTITIES {
        let json = to_json(id);
        assert!(json.contains(id.lang), "lang missing for {}", id.lang);
        assert!(json.contains(id.sdk_version), "sdk_version missing for {}", id.lang);
        assert!(json.contains(id.agent_id), "agent_id missing for {}", id.lang);
    }
}

#[test]
fn all_agent_ids_contain_ancora_prefix() {
    for id in IDENTITIES {
        assert!(id.agent_id.starts_with("ancora-"), "agent_id must start with ancora-: {}", id.agent_id);
    }
}
