/// Kimi (Moonshot AI) provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const KIMI_V1_5: &str         = "moonshot-v1-128k";
const KIMI_V1_8K: &str        = "moonshot-v1-8k";
const KIMI_V1_32K: &str       = "moonshot-v1-32k";

fn kimi_router() -> ModelRouter {
    let mut r = ModelRouter::new(KIMI_V1_5);
    r.bind("short-node", KIMI_V1_8K);
    r.bind("medium-node", KIMI_V1_32K);
    r
}

#[test]
fn kimi_default_is_128k() {
    assert_eq!(kimi_router().resolve("any", None), KIMI_V1_5);
}

#[test]
fn kimi_8k_binding_resolves() {
    assert_eq!(kimi_router().resolve("short-node", None), KIMI_V1_8K);
}

#[test]
fn kimi_32k_binding_resolves() {
    assert_eq!(kimi_router().resolve("medium-node", None), KIMI_V1_32K);
}

#[test]
fn kimi_unbound_falls_back_to_128k() {
    assert_eq!(kimi_router().resolve("unbound", None), KIMI_V1_5);
}

#[test]
fn kimi_model_strings_start_with_moonshot() {
    for m in [KIMI_V1_5, KIMI_V1_8K, KIMI_V1_32K] {
        assert!(m.starts_with("moonshot-"), "Expected moonshot- prefix in {m}");
    }
}

#[test]
fn kimi_all_models_distinct() {
    let models = [KIMI_V1_5, KIMI_V1_8K, KIMI_V1_32K];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn kimi_bind_small_assigns_8k() {
    let mut r = ModelRouter::new(KIMI_V1_5);
    r.bind_small(&["classify", "extract"], KIMI_V1_8K);
    assert_eq!(r.resolve("classify", None), KIMI_V1_8K);
    assert_eq!(r.resolve("plan", None), KIMI_V1_5);
}

#[test]
fn kimi_models_no_whitespace() {
    for m in [KIMI_V1_5, KIMI_V1_8K, KIMI_V1_32K] {
        assert_eq!(m.trim(), m);
    }
}
