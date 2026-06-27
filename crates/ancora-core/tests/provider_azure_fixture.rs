/// Azure OpenAI provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const AZURE_GPT4O: &str = "azure/gpt-4o";
const AZURE_GPT4O_MINI: &str = "azure/gpt-4o-mini";
const AZURE_GPT4_TURBO: &str = "azure/gpt-4-turbo";

fn azure_router() -> ModelRouter {
    let mut r = ModelRouter::new(AZURE_GPT4O);
    r.bind("mini-node", AZURE_GPT4O_MINI);
    r.bind("turbo-node", AZURE_GPT4_TURBO);
    r
}

#[test]
fn azure_default_is_gpt4o() {
    let r = ModelRouter::new(AZURE_GPT4O);
    assert_eq!(r.resolve("any", None), AZURE_GPT4O);
}

#[test]
fn azure_mini_binding_resolves() {
    assert_eq!(azure_router().resolve("mini-node", None), AZURE_GPT4O_MINI);
}

#[test]
fn azure_turbo_binding_resolves() {
    assert_eq!(azure_router().resolve("turbo-node", None), AZURE_GPT4_TURBO);
}

#[test]
fn azure_unbound_falls_back_to_default() {
    assert_eq!(azure_router().resolve("unbound", None), AZURE_GPT4O);
}

#[test]
fn azure_model_strings_contain_azure_prefix() {
    for m in [AZURE_GPT4O, AZURE_GPT4O_MINI, AZURE_GPT4_TURBO] {
        assert!(m.starts_with("azure/"), "Expected azure/ prefix in {m}");
    }
}

#[test]
fn azure_and_openai_model_strings_are_distinct() {
    assert_ne!(AZURE_GPT4O, "gpt-4o");
    assert_ne!(AZURE_GPT4O_MINI, "gpt-4o-mini");
}

#[test]
fn azure_rebind_takes_latest() {
    let mut r = ModelRouter::new(AZURE_GPT4O);
    r.bind("n", AZURE_GPT4O_MINI);
    r.bind("n", AZURE_GPT4_TURBO);
    assert_eq!(r.resolve("n", None), AZURE_GPT4_TURBO);
}

#[test]
fn azure_bind_small_assigns_mini() {
    let mut r = ModelRouter::new(AZURE_GPT4O);
    r.bind_small(&["cheap-1", "cheap-2"], AZURE_GPT4O_MINI);
    assert_eq!(r.resolve("cheap-1", None), AZURE_GPT4O_MINI);
    assert_eq!(r.resolve("expensive", None), AZURE_GPT4O);
}

#[test]
fn azure_models_no_whitespace() {
    for m in [AZURE_GPT4O, AZURE_GPT4O_MINI, AZURE_GPT4_TURBO] {
        assert_eq!(m.trim(), m);
    }
}
