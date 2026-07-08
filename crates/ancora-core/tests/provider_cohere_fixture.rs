/// Cohere provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const COMMAND_R_PLUS: &str = "command-r-plus";
const COMMAND_R: &str = "command-r";
const COMMAND_LIGHT: &str = "command-light";

fn cohere_router() -> ModelRouter {
    let mut r = ModelRouter::new(COMMAND_R_PLUS);
    r.bind("r-node", COMMAND_R);
    r.bind("light-node", COMMAND_LIGHT);
    r
}

#[test]
fn cohere_default_is_command_r_plus() {
    let r = ModelRouter::new(COMMAND_R_PLUS);
    assert_eq!(r.resolve("any", None), COMMAND_R_PLUS);
}

#[test]
fn command_r_binding_resolves() {
    assert_eq!(cohere_router().resolve("r-node", None), COMMAND_R);
}

#[test]
fn command_light_binding_resolves() {
    assert_eq!(cohere_router().resolve("light-node", None), COMMAND_LIGHT);
}

#[test]
fn cohere_unbound_falls_back_to_default() {
    assert_eq!(cohere_router().resolve("unbound", None), COMMAND_R_PLUS);
}

#[test]
fn cohere_all_models_start_with_command() {
    for m in [COMMAND_R_PLUS, COMMAND_R, COMMAND_LIGHT] {
        assert!(m.starts_with("command-"), "Expected command- prefix in {m}");
    }
}

#[test]
fn cohere_all_models_distinct() {
    let models = [COMMAND_R_PLUS, COMMAND_R, COMMAND_LIGHT];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn cohere_bind_small_assigns_light() {
    let mut r = ModelRouter::new(COMMAND_R_PLUS);
    r.bind_small(&["classify", "extract"], COMMAND_LIGHT);
    assert_eq!(r.resolve("classify", None), COMMAND_LIGHT);
    assert_eq!(r.resolve("synthesize", None), COMMAND_R_PLUS);
}

#[test]
fn cohere_models_no_whitespace() {
    for m in [COMMAND_R_PLUS, COMMAND_R, COMMAND_LIGHT] {
        assert_eq!(m.trim(), m);
    }
}
