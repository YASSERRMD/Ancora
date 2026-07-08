/// Anthropic provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const CLAUDE_OPUS: &str = "claude-opus-4-8";
const CLAUDE_SONNET: &str = "claude-sonnet-4-6";
const CLAUDE_HAIKU: &str = "claude-haiku-4-5";

fn anthropic_router() -> ModelRouter {
    let mut r = ModelRouter::new(CLAUDE_OPUS);
    r.bind("sonnet-node", CLAUDE_SONNET);
    r.bind("haiku-node", CLAUDE_HAIKU);
    r
}

#[test]
fn anthropic_default_is_opus() {
    let r = ModelRouter::new(CLAUDE_OPUS);
    assert_eq!(r.resolve("any", None), CLAUDE_OPUS);
}

#[test]
fn anthropic_sonnet_binding_resolves() {
    assert_eq!(
        anthropic_router().resolve("sonnet-node", None),
        CLAUDE_SONNET
    );
}

#[test]
fn anthropic_haiku_binding_resolves() {
    assert_eq!(anthropic_router().resolve("haiku-node", None), CLAUDE_HAIKU);
}

#[test]
fn anthropic_unbound_falls_back_to_opus() {
    assert_eq!(anthropic_router().resolve("plan-node", None), CLAUDE_OPUS);
}

#[test]
fn anthropic_model_strings_start_with_claude() {
    for m in [CLAUDE_OPUS, CLAUDE_SONNET, CLAUDE_HAIKU] {
        assert!(m.starts_with("claude-"), "Expected claude- prefix in {m}");
    }
}

#[test]
fn anthropic_all_models_distinct() {
    let models = [CLAUDE_OPUS, CLAUDE_SONNET, CLAUDE_HAIKU];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn haiku_assigned_to_cheap_nodes() {
    let mut r = ModelRouter::new(CLAUDE_OPUS);
    r.bind_small(&["classify", "triage", "route"], CLAUDE_HAIKU);
    for node in ["classify", "triage", "route"] {
        assert_eq!(r.resolve(node, None), CLAUDE_HAIKU);
    }
    assert_eq!(r.resolve("plan", None), CLAUDE_OPUS);
}

#[test]
fn rebind_sonnet_wins() {
    let mut r = ModelRouter::new(CLAUDE_OPUS);
    r.bind("n", CLAUDE_HAIKU);
    r.bind("n", CLAUDE_SONNET);
    assert_eq!(r.resolve("n", None), CLAUDE_SONNET);
}

#[test]
fn anthropic_models_no_whitespace() {
    for m in [CLAUDE_OPUS, CLAUDE_SONNET, CLAUDE_HAIKU] {
        assert_eq!(m.trim(), m);
    }
}
