/// Residency tags per provider -- validates model-to-region mapping.
/// Offline, no HTTP calls.
use ancora_core::routing::ModelRouter;
use std::collections::HashMap;

/// Associates provider models with their allowed data residency regions.
fn build_residency_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert("claude-sonnet-4-6", vec!["us-east-1", "eu-west-1"]);
    map.insert("claude-opus-4-8", vec!["us-east-1", "eu-west-1"]);
    map.insert("gpt-4o", vec!["us-east-1", "us-west-2", "eu-west-1"]);
    map.insert("gemini-2-5-pro", vec!["us-central1", "eu-west4"]);
    map.insert("deepseek-chat", vec!["cn-beijing", "cn-hangzhou"]);
    map.insert("qwen3-max", vec!["cn-hangzhou", "ap-southeast-1"]);
    map.insert("glm-5", vec!["cn-beijing"]);
    map.insert("moonshot-v1-128k", vec!["cn-beijing"]);
    map
}

#[test]
fn claude_has_us_and_eu_regions() {
    let map = build_residency_map();
    let regions = map.get("claude-sonnet-4-6").unwrap();
    assert!(regions.contains(&"us-east-1"));
    assert!(regions.contains(&"eu-west-1"));
}

#[test]
fn gpt4o_has_three_regions() {
    let map = build_residency_map();
    assert_eq!(map.get("gpt-4o").unwrap().len(), 3);
}

#[test]
fn deepseek_is_china_only() {
    let map = build_residency_map();
    let regions = map.get("deepseek-chat").unwrap();
    for r in regions {
        assert!(r.starts_with("cn-"), "DeepSeek must be China-only, got {r}");
    }
}

#[test]
fn qwen_has_intl_region() {
    let map = build_residency_map();
    let regions = map.get("qwen3-max").unwrap();
    assert!(
        regions.contains(&"ap-southeast-1"),
        "Qwen should have international endpoint"
    );
}

#[test]
fn glm_is_china_only() {
    let map = build_residency_map();
    let regions = map.get("glm-5").unwrap();
    assert_eq!(regions.len(), 1);
    assert!(regions[0].starts_with("cn-"));
}

#[test]
fn all_providers_have_at_least_one_region() {
    let map = build_residency_map();
    for (model, regions) in &map {
        assert!(!regions.is_empty(), "Model {model} has no regions");
    }
}

#[test]
fn eu_providers_have_eu_regions() {
    let map = build_residency_map();
    for model in ["claude-sonnet-4-6", "gpt-4o", "gemini-2-5-pro"] {
        let regions = map.get(model).unwrap();
        let has_eu = regions.iter().any(|r| r.starts_with("eu"));
        assert!(has_eu, "Model {model} should have an EU region");
    }
}

#[test]
fn router_with_eu_default_resolves_correctly() {
    let mut router = ModelRouter::new("claude-sonnet-4-6");
    router.bind("local-eu-node", "gemini-2-5-pro");
    assert_eq!(router.resolve("local-eu-node", None), "gemini-2-5-pro");
    assert_eq!(router.resolve("unbound", None), "claude-sonnet-4-6");
}

#[test]
fn china_providers_not_in_eu_regions() {
    let map = build_residency_map();
    for model in ["deepseek-chat", "glm-5"] {
        let regions = map.get(model).unwrap();
        let has_eu = regions.iter().any(|r| r.starts_with("eu"));
        assert!(
            !has_eu,
            "Model {model} should not have EU regions, got {regions:?}"
        );
    }
}

#[test]
fn eight_providers_in_residency_map() {
    let map = build_residency_map();
    assert_eq!(map.len(), 8);
}
