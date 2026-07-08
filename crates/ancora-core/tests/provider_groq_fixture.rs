/// Groq provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const GROQ_LLAMA3_70B: &str = "groq/llama3-70b-8192";
const GROQ_LLAMA3_8B: &str = "groq/llama3-8b-8192";
const GROQ_MIXTRAL: &str = "groq/mixtral-8x7b-32768";
const GROQ_GEMMA2: &str = "groq/gemma2-9b-it";

fn groq_router() -> ModelRouter {
    let mut r = ModelRouter::new(GROQ_LLAMA3_70B);
    r.bind("llama-small", GROQ_LLAMA3_8B);
    r.bind("mixtral-node", GROQ_MIXTRAL);
    r.bind("gemma-node", GROQ_GEMMA2);
    r
}

#[test]
fn groq_default_is_llama3_70b() {
    let r = ModelRouter::new(GROQ_LLAMA3_70B);
    assert_eq!(r.resolve("any", None), GROQ_LLAMA3_70B);
}

#[test]
fn groq_llama3_8b_binding_resolves() {
    assert_eq!(groq_router().resolve("llama-small", None), GROQ_LLAMA3_8B);
}

#[test]
fn groq_mixtral_binding_resolves() {
    assert_eq!(groq_router().resolve("mixtral-node", None), GROQ_MIXTRAL);
}

#[test]
fn groq_gemma2_binding_resolves() {
    assert_eq!(groq_router().resolve("gemma-node", None), GROQ_GEMMA2);
}

#[test]
fn groq_unbound_falls_back_to_default() {
    assert_eq!(groq_router().resolve("unbound", None), GROQ_LLAMA3_70B);
}

#[test]
fn groq_model_strings_have_prefix() {
    for m in [GROQ_LLAMA3_70B, GROQ_LLAMA3_8B, GROQ_MIXTRAL, GROQ_GEMMA2] {
        assert!(m.starts_with("groq/"), "Expected groq/ prefix in {m}");
    }
}

#[test]
fn groq_all_models_distinct() {
    let models = [GROQ_LLAMA3_70B, GROQ_LLAMA3_8B, GROQ_MIXTRAL, GROQ_GEMMA2];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn groq_bind_small_assigns_8b() {
    let mut r = ModelRouter::new(GROQ_LLAMA3_70B);
    r.bind_small(&["fast-a", "fast-b"], GROQ_LLAMA3_8B);
    assert_eq!(r.resolve("fast-a", None), GROQ_LLAMA3_8B);
    assert_eq!(r.resolve("complex", None), GROQ_LLAMA3_70B);
}

#[test]
fn groq_models_no_whitespace() {
    for m in [GROQ_LLAMA3_70B, GROQ_LLAMA3_8B, GROQ_MIXTRAL, GROQ_GEMMA2] {
        assert_eq!(m.trim(), m);
    }
}
