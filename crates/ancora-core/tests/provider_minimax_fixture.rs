/// MiniMax provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const MINIMAX_TEXT_01: &str  = "minimax/abab7-chat-preview";
const MINIMAX_TEXT_02: &str  = "minimax/abab6.5s-chat";
const MINIMAX_VIDEO: &str    = "minimax/video-01";

fn minimax_router() -> ModelRouter {
    let mut r = ModelRouter::new(MINIMAX_TEXT_01);
    r.bind("lite-node", MINIMAX_TEXT_02);
    r.bind("video-node", MINIMAX_VIDEO);
    r
}

#[test]
fn minimax_default_is_text_01() {
    assert_eq!(minimax_router().resolve("any", None), MINIMAX_TEXT_01);
}

#[test]
fn minimax_text_02_binding_resolves() {
    assert_eq!(minimax_router().resolve("lite-node", None), MINIMAX_TEXT_02);
}

#[test]
fn minimax_video_binding_resolves() {
    assert_eq!(minimax_router().resolve("video-node", None), MINIMAX_VIDEO);
}

#[test]
fn minimax_unbound_falls_back_to_default() {
    assert_eq!(minimax_router().resolve("unbound", None), MINIMAX_TEXT_01);
}

#[test]
fn minimax_model_strings_have_prefix() {
    for m in [MINIMAX_TEXT_01, MINIMAX_TEXT_02, MINIMAX_VIDEO] {
        assert!(m.starts_with("minimax/"), "Expected minimax/ prefix in {m}");
    }
}

#[test]
fn minimax_all_models_distinct() {
    let models = [MINIMAX_TEXT_01, MINIMAX_TEXT_02, MINIMAX_VIDEO];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn minimax_bind_small_assigns_lite() {
    let mut r = ModelRouter::new(MINIMAX_TEXT_01);
    r.bind_small(&["fast-1"], MINIMAX_TEXT_02);
    assert_eq!(r.resolve("fast-1", None), MINIMAX_TEXT_02);
    assert_eq!(r.resolve("plan", None), MINIMAX_TEXT_01);
}

#[test]
fn minimax_models_no_whitespace() {
    for m in [MINIMAX_TEXT_01, MINIMAX_TEXT_02, MINIMAX_VIDEO] {
        assert_eq!(m.trim(), m);
    }
}
