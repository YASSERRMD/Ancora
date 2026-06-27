/// OpenAI provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const GPT4O: &str = "gpt-4o";
const GPT4O_MINI: &str = "gpt-4o-mini";
const GPT4_TURBO: &str = "gpt-4-turbo";
const O1_PREVIEW: &str = "o1-preview";

fn openai_router() -> ModelRouter {
    let mut r = ModelRouter::new(GPT4O);
    r.bind("mini-node", GPT4O_MINI);
    r.bind("turbo-node", GPT4_TURBO);
    r.bind("o1-node", O1_PREVIEW);
    r
}

#[test]
fn openai_default_is_gpt4o() {
    let r = ModelRouter::new(GPT4O);
    assert_eq!(r.resolve("any", None), GPT4O);
}

#[test]
fn gpt4o_mini_binding_resolves() {
    assert_eq!(openai_router().resolve("mini-node", None), GPT4O_MINI);
}

#[test]
fn gpt4_turbo_binding_resolves() {
    assert_eq!(openai_router().resolve("turbo-node", None), GPT4_TURBO);
}

#[test]
fn o1_preview_binding_resolves() {
    assert_eq!(openai_router().resolve("o1-node", None), O1_PREVIEW);
}

#[test]
fn unbound_openai_node_falls_back_to_gpt4o() {
    assert_eq!(openai_router().resolve("unknown-node", None), GPT4O);
}

#[test]
fn node_override_beats_default_not_binding() {
    assert_eq!(openai_router().resolve("mini-node", Some(GPT4_TURBO)), GPT4O_MINI);
    assert_eq!(openai_router().resolve("unbound", Some(GPT4_TURBO)), GPT4_TURBO);
}

#[test]
fn all_openai_models_are_distinct() {
    let models = [GPT4O, GPT4O_MINI, GPT4_TURBO, O1_PREVIEW];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn rebind_openai_node_takes_latest() {
    let mut r = ModelRouter::new(GPT4O);
    r.bind("n", GPT4O_MINI);
    r.bind("n", O1_PREVIEW);
    assert_eq!(r.resolve("n", None), O1_PREVIEW);
}

#[test]
fn bind_small_assigns_mini_to_cheap_nodes() {
    let mut r = ModelRouter::new(GPT4O);
    r.bind_small(&["classify", "triage"], GPT4O_MINI);
    assert_eq!(r.resolve("classify", None), GPT4O_MINI);
    assert_eq!(r.resolve("triage", None), GPT4O_MINI);
    assert_eq!(r.resolve("plan", None), GPT4O);
}

#[test]
fn openai_model_strings_have_no_whitespace() {
    for m in [GPT4O, GPT4O_MINI, GPT4_TURBO, O1_PREVIEW] {
        assert_eq!(m.trim(), m);
    }
}
