/// AWS Bedrock provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const BEDROCK_CLAUDE_SONNET: &str = "bedrock/anthropic.claude-3-5-sonnet-20241022-v2:0";
const BEDROCK_CLAUDE_HAIKU: &str = "bedrock/anthropic.claude-3-haiku-20240307-v1:0";
const BEDROCK_LLAMA3: &str = "bedrock/meta.llama3-70b-instruct-v1:0";
const BEDROCK_TITAN: &str = "bedrock/amazon.titan-text-express-v1";

fn bedrock_router() -> ModelRouter {
    let mut r = ModelRouter::new(BEDROCK_CLAUDE_SONNET);
    r.bind("haiku-node", BEDROCK_CLAUDE_HAIKU);
    r.bind("llama-node", BEDROCK_LLAMA3);
    r.bind("titan-node", BEDROCK_TITAN);
    r
}

#[test]
fn bedrock_default_is_claude_sonnet() {
    let r = ModelRouter::new(BEDROCK_CLAUDE_SONNET);
    assert_eq!(r.resolve("any", None), BEDROCK_CLAUDE_SONNET);
}

#[test]
fn bedrock_haiku_binding_resolves() {
    assert_eq!(
        bedrock_router().resolve("haiku-node", None),
        BEDROCK_CLAUDE_HAIKU
    );
}

#[test]
fn bedrock_llama3_binding_resolves() {
    assert_eq!(bedrock_router().resolve("llama-node", None), BEDROCK_LLAMA3);
}

#[test]
fn bedrock_titan_binding_resolves() {
    assert_eq!(bedrock_router().resolve("titan-node", None), BEDROCK_TITAN);
}

#[test]
fn bedrock_unbound_falls_back_to_default() {
    assert_eq!(
        bedrock_router().resolve("unbound", None),
        BEDROCK_CLAUDE_SONNET
    );
}

#[test]
fn bedrock_model_strings_have_prefix() {
    for m in [
        BEDROCK_CLAUDE_SONNET,
        BEDROCK_CLAUDE_HAIKU,
        BEDROCK_LLAMA3,
        BEDROCK_TITAN,
    ] {
        assert!(m.starts_with("bedrock/"), "Expected bedrock/ prefix in {m}");
    }
}

#[test]
fn bedrock_all_models_distinct() {
    let models = [
        BEDROCK_CLAUDE_SONNET,
        BEDROCK_CLAUDE_HAIKU,
        BEDROCK_LLAMA3,
        BEDROCK_TITAN,
    ];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 4);
}

#[test]
fn bedrock_bind_small_assigns_haiku() {
    let mut r = ModelRouter::new(BEDROCK_CLAUDE_SONNET);
    r.bind_small(&["triage", "route"], BEDROCK_CLAUDE_HAIKU);
    assert_eq!(r.resolve("triage", None), BEDROCK_CLAUDE_HAIKU);
    assert_eq!(r.resolve("plan", None), BEDROCK_CLAUDE_SONNET);
}

#[test]
fn bedrock_models_no_whitespace() {
    for m in [
        BEDROCK_CLAUDE_SONNET,
        BEDROCK_CLAUDE_HAIKU,
        BEDROCK_LLAMA3,
        BEDROCK_TITAN,
    ] {
        assert_eq!(m.trim(), m);
    }
}
