/// Fireworks AI provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const FW_LLAMA3_70B: &str = "fireworks_ai/accounts/fireworks/models/llama-v3-70b-instruct";
const FW_LLAMA3_8B: &str = "fireworks_ai/accounts/fireworks/models/llama-v3-8b-instruct";
const FW_MIXTRAL: &str = "fireworks_ai/accounts/fireworks/models/mixtral-8x7b-instruct";
const FW_DEEPSEEK: &str = "fireworks_ai/accounts/fireworks/models/deepseek-coder-33b-instruct";

fn fireworks_router() -> ModelRouter {
    let mut r = ModelRouter::new(FW_LLAMA3_70B);
    r.bind("small-node", FW_LLAMA3_8B);
    r.bind("mixtral-node", FW_MIXTRAL);
    r.bind("code-node", FW_DEEPSEEK);
    r
}

#[test]
fn fireworks_default_is_llama3_70b() {
    assert_eq!(fireworks_router().resolve("any", None), FW_LLAMA3_70B);
}

#[test]
fn fireworks_llama3_8b_binding_resolves() {
    assert_eq!(fireworks_router().resolve("small-node", None), FW_LLAMA3_8B);
}

#[test]
fn fireworks_mixtral_binding_resolves() {
    assert_eq!(fireworks_router().resolve("mixtral-node", None), FW_MIXTRAL);
}

#[test]
fn fireworks_deepseek_binding_resolves() {
    assert_eq!(fireworks_router().resolve("code-node", None), FW_DEEPSEEK);
}

#[test]
fn fireworks_model_strings_have_prefix() {
    for m in [FW_LLAMA3_70B, FW_LLAMA3_8B, FW_MIXTRAL, FW_DEEPSEEK] {
        assert!(
            m.starts_with("fireworks_ai/"),
            "Expected fireworks_ai/ prefix in {m}"
        );
    }
}

#[test]
fn fireworks_all_models_distinct() {
    let models = [FW_LLAMA3_70B, FW_LLAMA3_8B, FW_MIXTRAL, FW_DEEPSEEK];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn fireworks_bind_small_assigns_8b() {
    let mut r = ModelRouter::new(FW_LLAMA3_70B);
    r.bind_small(&["fast-a", "fast-b"], FW_LLAMA3_8B);
    assert_eq!(r.resolve("fast-a", None), FW_LLAMA3_8B);
    assert_eq!(r.resolve("plan", None), FW_LLAMA3_70B);
}

#[test]
fn fireworks_models_no_whitespace() {
    for m in [FW_LLAMA3_70B, FW_LLAMA3_8B, FW_MIXTRAL, FW_DEEPSEEK] {
        assert_eq!(m.trim(), m);
    }
}
