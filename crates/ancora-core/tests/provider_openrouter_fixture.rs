/// OpenRouter provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const OR_LLAMA3: &str = "openrouter/meta-llama/llama-3-70b-instruct";
const OR_MISTRAL: &str = "openrouter/mistralai/mistral-7b-instruct";
const OR_CLAUDE: &str = "openrouter/anthropic/claude-3-5-sonnet";
const OR_GEMINI: &str = "openrouter/google/gemini-pro";

fn openrouter_router() -> ModelRouter {
    let mut r = ModelRouter::new(OR_LLAMA3);
    r.bind("mistral-node", OR_MISTRAL);
    r.bind("claude-node", OR_CLAUDE);
    r.bind("gemini-node", OR_GEMINI);
    r
}

#[test]
fn openrouter_default_is_llama3() {
    let r = ModelRouter::new(OR_LLAMA3);
    assert_eq!(r.resolve("any", None), OR_LLAMA3);
}

#[test]
fn openrouter_mistral_binding_resolves() {
    assert_eq!(
        openrouter_router().resolve("mistral-node", None),
        OR_MISTRAL
    );
}

#[test]
fn openrouter_claude_binding_resolves() {
    assert_eq!(openrouter_router().resolve("claude-node", None), OR_CLAUDE);
}

#[test]
fn openrouter_gemini_binding_resolves() {
    assert_eq!(openrouter_router().resolve("gemini-node", None), OR_GEMINI);
}

#[test]
fn openrouter_unbound_falls_back_to_default() {
    assert_eq!(openrouter_router().resolve("unbound", None), OR_LLAMA3);
}

#[test]
fn openrouter_model_strings_have_prefix() {
    for m in [OR_LLAMA3, OR_MISTRAL, OR_CLAUDE, OR_GEMINI] {
        assert!(
            m.starts_with("openrouter/"),
            "Expected openrouter/ prefix in {m}"
        );
    }
}

#[test]
fn openrouter_all_models_distinct() {
    let models = [OR_LLAMA3, OR_MISTRAL, OR_CLAUDE, OR_GEMINI];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn openrouter_bind_small_assigns_mistral() {
    let mut r = ModelRouter::new(OR_LLAMA3);
    r.bind_small(&["fast-1", "fast-2"], OR_MISTRAL);
    assert_eq!(r.resolve("fast-1", None), OR_MISTRAL);
    assert_eq!(r.resolve("complex", None), OR_LLAMA3);
}

#[test]
fn openrouter_models_no_whitespace() {
    for m in [OR_LLAMA3, OR_MISTRAL, OR_CLAUDE, OR_GEMINI] {
        assert_eq!(m.trim(), m);
    }
}
