/// Gateway routing fixtures -- offline, no HTTP calls.
/// Tests that a gateway can route to multiple providers and fallback correctly.
use ancora_core::routing::ModelRouter;

const GPT4O: &str = "gpt-4o";
const CLAUDE_SONNET: &str = "claude-sonnet-4-6";
const GEMINI_25_PRO: &str = "gemini-2-5-pro";
const DEEPSEEK_CHAT: &str = "deepseek-chat";
const QWEN3_MAX: &str = "qwen3-max";
const GLM5: &str = "glm-5";
const LLAMA3: &str = "llama3";

fn gateway_router() -> ModelRouter {
    let mut r = ModelRouter::new(GPT4O);
    r.bind("claude-gw", CLAUDE_SONNET);
    r.bind("gemini-gw", GEMINI_25_PRO);
    r.bind("deepseek-gw", DEEPSEEK_CHAT);
    r.bind("qwen-gw", QWEN3_MAX);
    r.bind("glm-gw", GLM5);
    r.bind("llama-gw", LLAMA3);
    r
}

#[test]
fn gateway_default_is_gpt4o() {
    assert_eq!(gateway_router().resolve("any", None), GPT4O);
}

#[test]
fn gateway_routes_to_claude() {
    assert_eq!(gateway_router().resolve("claude-gw", None), CLAUDE_SONNET);
}

#[test]
fn gateway_routes_to_gemini() {
    assert_eq!(gateway_router().resolve("gemini-gw", None), GEMINI_25_PRO);
}

#[test]
fn gateway_routes_to_deepseek() {
    assert_eq!(gateway_router().resolve("deepseek-gw", None), DEEPSEEK_CHAT);
}

#[test]
fn gateway_routes_to_qwen() {
    assert_eq!(gateway_router().resolve("qwen-gw", None), QWEN3_MAX);
}

#[test]
fn gateway_routes_to_glm() {
    assert_eq!(gateway_router().resolve("glm-gw", None), GLM5);
}

#[test]
fn gateway_routes_to_llama() {
    assert_eq!(gateway_router().resolve("llama-gw", None), LLAMA3);
}

#[test]
fn gateway_unbound_falls_back_to_gpt4o() {
    assert_eq!(gateway_router().resolve("unknown-provider", None), GPT4O);
}

#[test]
fn gateway_node_override_used_when_no_binding() {
    assert_eq!(
        gateway_router().resolve("no-binding", Some(DEEPSEEK_CHAT)),
        DEEPSEEK_CHAT
    );
}

#[test]
fn gateway_binding_beats_node_override() {
    assert_eq!(
        gateway_router().resolve("claude-gw", Some(LLAMA3)),
        CLAUDE_SONNET
    );
}

#[test]
fn gateway_seven_providers_all_distinct() {
    let r = gateway_router();
    let models = [
        r.resolve("any", None),
        r.resolve("claude-gw", None),
        r.resolve("gemini-gw", None),
        r.resolve("deepseek-gw", None),
        r.resolve("qwen-gw", None),
        r.resolve("glm-gw", None),
        r.resolve("llama-gw", None),
    ];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 7);
}
