/// End-to-end provider mapping tests (offline).
///
/// Validates that the ModelRouter maps node IDs to the correct provider
/// model strings for five distinct providers: Anthropic, OpenAI, Gemini,
/// Mistral, and DeepSeek. All tests run offline with no HTTP calls.
use ancora_core::routing::ModelRouter;

const ANTHROPIC: &str = "claude-opus-4-8";
const OPENAI: &str = "gpt-4o";
const GEMINI: &str = "gemini-2-5-pro";
const MISTRAL: &str = "mistral-large-latest";
const DEEPSEEK: &str = "deepseek-chat";

fn five_provider_router() -> ModelRouter {
    let mut router = ModelRouter::new(ANTHROPIC);
    router
        .bind("node-openai", OPENAI)
        .bind("node-gemini", GEMINI)
        .bind("node-mistral", MISTRAL)
        .bind("node-deepseek", DEEPSEEK);
    router
}

#[test]
fn default_provider_is_anthropic() {
    let router = ModelRouter::new(ANTHROPIC);
    assert_eq!(router.resolve("any-node", None), ANTHROPIC);
}

#[test]
fn openai_binding_resolves_correctly() {
    let router = five_provider_router();
    assert_eq!(router.resolve("node-openai", None), OPENAI);
}

#[test]
fn gemini_binding_resolves_correctly() {
    let router = five_provider_router();
    assert_eq!(router.resolve("node-gemini", None), GEMINI);
}

#[test]
fn mistral_binding_resolves_correctly() {
    let router = five_provider_router();
    assert_eq!(router.resolve("node-mistral", None), MISTRAL);
}

#[test]
fn deepseek_binding_resolves_correctly() {
    let router = five_provider_router();
    assert_eq!(router.resolve("node-deepseek", None), DEEPSEEK);
}

#[test]
fn unbound_node_falls_back_to_anthropic() {
    let router = five_provider_router();
    assert_eq!(router.resolve("node-unbound", None), ANTHROPIC);
}

#[test]
fn node_level_override_beats_default_but_loses_to_binding() {
    let mut router = ModelRouter::new(ANTHROPIC);
    router.bind("node-bound", OPENAI);
    assert_eq!(router.resolve("node-bound", Some(GEMINI)), OPENAI);
    assert_eq!(router.resolve("node-unbound", Some(GEMINI)), GEMINI);
}

#[test]
fn bind_small_assigns_mistral_to_cheap_nodes() {
    let mut router = ModelRouter::new(ANTHROPIC);
    router.bind_small(&["classify", "summarize", "extract"], MISTRAL);
    assert_eq!(router.resolve("classify", None), MISTRAL);
    assert_eq!(router.resolve("summarize", None), MISTRAL);
    assert_eq!(router.resolve("extract", None), MISTRAL);
    assert_eq!(router.resolve("plan", None), ANTHROPIC);
}

#[test]
fn provider_binding_survives_rebind() {
    let mut router = ModelRouter::new(ANTHROPIC);
    router.bind("node-a", OPENAI);
    router.bind("node-a", DEEPSEEK);
    assert_eq!(
        router.resolve("node-a", None),
        DEEPSEEK,
        "last bind must win"
    );
}

#[test]
fn all_five_providers_resolve_to_different_models() {
    let router = five_provider_router();
    let models = [
        router.resolve("node-default", None),
        router.resolve("node-openai", None),
        router.resolve("node-gemini", None),
        router.resolve("node-mistral", None),
        router.resolve("node-deepseek", None),
    ];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(
        unique.len(),
        5,
        "all five providers must map to distinct model strings"
    );
}

#[test]
fn empty_node_model_id_falls_back_to_default() {
    let router = ModelRouter::new(ANTHROPIC);
    assert_eq!(router.resolve("n", Some("")), ANTHROPIC);
}
