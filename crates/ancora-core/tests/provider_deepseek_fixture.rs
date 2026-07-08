/// DeepSeek provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const DEEPSEEK_CHAT: &str = "deepseek-chat";
const DEEPSEEK_CODER: &str = "deepseek-coder";
const DEEPSEEK_R1: &str = "deepseek-reasoner";

fn deepseek_router() -> ModelRouter {
    let mut r = ModelRouter::new(DEEPSEEK_CHAT);
    r.bind("coder-node", DEEPSEEK_CODER);
    r.bind("r1-node", DEEPSEEK_R1);
    r
}

#[test]
fn deepseek_default_is_chat() {
    assert_eq!(deepseek_router().resolve("any", None), DEEPSEEK_CHAT);
}

#[test]
fn deepseek_coder_binding_resolves() {
    assert_eq!(
        deepseek_router().resolve("coder-node", None),
        DEEPSEEK_CODER
    );
}

#[test]
fn deepseek_r1_binding_resolves() {
    assert_eq!(deepseek_router().resolve("r1-node", None), DEEPSEEK_R1);
}

#[test]
fn deepseek_unbound_falls_back_to_chat() {
    assert_eq!(deepseek_router().resolve("unbound", None), DEEPSEEK_CHAT);
}

#[test]
fn deepseek_all_models_start_with_deepseek() {
    for m in [DEEPSEEK_CHAT, DEEPSEEK_CODER, DEEPSEEK_R1] {
        assert!(
            m.starts_with("deepseek-"),
            "Expected deepseek- prefix in {m}"
        );
    }
}

#[test]
fn deepseek_all_models_distinct() {
    let models = [DEEPSEEK_CHAT, DEEPSEEK_CODER, DEEPSEEK_R1];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn deepseek_bind_small_assigns_coder() {
    let mut r = ModelRouter::new(DEEPSEEK_CHAT);
    r.bind_small(&["code-review", "code-gen"], DEEPSEEK_CODER);
    assert_eq!(r.resolve("code-review", None), DEEPSEEK_CODER);
    assert_eq!(r.resolve("chat", None), DEEPSEEK_CHAT);
}

#[test]
fn deepseek_models_no_whitespace() {
    for m in [DEEPSEEK_CHAT, DEEPSEEK_CODER, DEEPSEEK_R1] {
        assert_eq!(m.trim(), m);
    }
}

#[test]
fn deepseek_rebind_takes_latest() {
    let mut r = ModelRouter::new(DEEPSEEK_CHAT);
    r.bind("n", DEEPSEEK_CODER);
    r.bind("n", DEEPSEEK_R1);
    assert_eq!(r.resolve("n", None), DEEPSEEK_R1);
}
