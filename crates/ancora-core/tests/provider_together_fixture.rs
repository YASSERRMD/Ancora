/// Together AI provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const TOGETHER_LLAMA3_70B: &str = "together_ai/meta-llama/Llama-3-70b-chat-hf";
const TOGETHER_LLAMA3_8B: &str = "together_ai/meta-llama/Llama-3-8b-chat-hf";
const TOGETHER_MIXTRAL: &str = "together_ai/mistralai/Mixtral-8x7B-Instruct-v0.1";
const TOGETHER_QWEN2: &str = "together_ai/Qwen/Qwen2-72B-Instruct";

fn together_router() -> ModelRouter {
    let mut r = ModelRouter::new(TOGETHER_LLAMA3_70B);
    r.bind("llama-small", TOGETHER_LLAMA3_8B);
    r.bind("mixtral-node", TOGETHER_MIXTRAL);
    r.bind("qwen-node", TOGETHER_QWEN2);
    r
}

#[test]
fn together_default_is_llama3_70b() {
    assert_eq!(together_router().resolve("any", None), TOGETHER_LLAMA3_70B);
}

#[test]
fn together_llama3_8b_binding_resolves() {
    assert_eq!(
        together_router().resolve("llama-small", None),
        TOGETHER_LLAMA3_8B
    );
}

#[test]
fn together_mixtral_binding_resolves() {
    assert_eq!(
        together_router().resolve("mixtral-node", None),
        TOGETHER_MIXTRAL
    );
}

#[test]
fn together_qwen2_binding_resolves() {
    assert_eq!(together_router().resolve("qwen-node", None), TOGETHER_QWEN2);
}

#[test]
fn together_model_strings_have_prefix() {
    for m in [
        TOGETHER_LLAMA3_70B,
        TOGETHER_LLAMA3_8B,
        TOGETHER_MIXTRAL,
        TOGETHER_QWEN2,
    ] {
        assert!(
            m.starts_with("together_ai/"),
            "Expected together_ai/ prefix in {m}"
        );
    }
}

#[test]
fn together_all_models_distinct() {
    let models = [
        TOGETHER_LLAMA3_70B,
        TOGETHER_LLAMA3_8B,
        TOGETHER_MIXTRAL,
        TOGETHER_QWEN2,
    ];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn together_bind_small_assigns_8b() {
    let mut r = ModelRouter::new(TOGETHER_LLAMA3_70B);
    r.bind_small(&["cheap-1", "cheap-2"], TOGETHER_LLAMA3_8B);
    assert_eq!(r.resolve("cheap-1", None), TOGETHER_LLAMA3_8B);
    assert_eq!(r.resolve("plan", None), TOGETHER_LLAMA3_70B);
}

#[test]
fn together_models_no_whitespace() {
    for m in [
        TOGETHER_LLAMA3_70B,
        TOGETHER_LLAMA3_8B,
        TOGETHER_MIXTRAL,
        TOGETHER_QWEN2,
    ] {
        assert_eq!(m.trim(), m);
    }
}

#[test]
fn together_rebind_takes_latest() {
    let mut r = ModelRouter::new(TOGETHER_LLAMA3_70B);
    r.bind("n", TOGETHER_LLAMA3_8B);
    r.bind("n", TOGETHER_QWEN2);
    assert_eq!(r.resolve("n", None), TOGETHER_QWEN2);
}
