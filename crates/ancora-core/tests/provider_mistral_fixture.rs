/// Mistral provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const MISTRAL_LARGE: &str   = "mistral-large-latest";
const MISTRAL_SMALL: &str   = "mistral-small-latest";
const MISTRAL_7B: &str      = "open-mistral-7b";
const CODESTRAL: &str       = "codestral-latest";

fn mistral_router() -> ModelRouter {
    let mut r = ModelRouter::new(MISTRAL_LARGE);
    r.bind("small-node", MISTRAL_SMALL);
    r.bind("7b-node", MISTRAL_7B);
    r.bind("code-node", CODESTRAL);
    r
}

#[test]
fn mistral_default_is_large() {
    let r = ModelRouter::new(MISTRAL_LARGE);
    assert_eq!(r.resolve("any", None), MISTRAL_LARGE);
}

#[test]
fn mistral_small_binding_resolves() {
    assert_eq!(mistral_router().resolve("small-node", None), MISTRAL_SMALL);
}

#[test]
fn mistral_7b_binding_resolves() {
    assert_eq!(mistral_router().resolve("7b-node", None), MISTRAL_7B);
}

#[test]
fn codestral_binding_resolves() {
    assert_eq!(mistral_router().resolve("code-node", None), CODESTRAL);
}

#[test]
fn mistral_unbound_falls_back_to_large() {
    assert_eq!(mistral_router().resolve("unbound", None), MISTRAL_LARGE);
}

#[test]
fn mistral_all_models_distinct() {
    let models = [MISTRAL_LARGE, MISTRAL_SMALL, MISTRAL_7B, CODESTRAL];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn mistral_bind_small_assigns_7b() {
    let mut r = ModelRouter::new(MISTRAL_LARGE);
    r.bind_small(&["summarize", "classify"], MISTRAL_7B);
    assert_eq!(r.resolve("summarize", None), MISTRAL_7B);
    assert_eq!(r.resolve("plan", None), MISTRAL_LARGE);
}

#[test]
fn mistral_models_no_whitespace() {
    for m in [MISTRAL_LARGE, MISTRAL_SMALL, MISTRAL_7B, CODESTRAL] {
        assert_eq!(m.trim(), m);
    }
}

#[test]
fn mistral_rebind_takes_latest() {
    let mut r = ModelRouter::new(MISTRAL_LARGE);
    r.bind("n", MISTRAL_SMALL);
    r.bind("n", CODESTRAL);
    assert_eq!(r.resolve("n", None), CODESTRAL);
}
