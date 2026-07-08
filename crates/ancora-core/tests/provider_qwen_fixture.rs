/// Qwen (Alibaba) provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const QWEN3_MAX: &str = "qwen3-max";
const QWEN3_PLUS: &str = "qwen3-plus";
const QWEN_TURBO: &str = "qwen-turbo";
const QWEN_LONG: &str = "qwen-long";
const QWEN_VL_PLUS: &str = "qwen-vl-plus";

fn qwen_router() -> ModelRouter {
    let mut r = ModelRouter::new(QWEN3_MAX);
    r.bind("plus-node", QWEN3_PLUS);
    r.bind("turbo-node", QWEN_TURBO);
    r.bind("long-node", QWEN_LONG);
    r.bind("vl-node", QWEN_VL_PLUS);
    r
}

#[test]
fn qwen_default_is_qwen3_max() {
    assert_eq!(qwen_router().resolve("any", None), QWEN3_MAX);
}

#[test]
fn qwen3_plus_binding_resolves() {
    assert_eq!(qwen_router().resolve("plus-node", None), QWEN3_PLUS);
}

#[test]
fn qwen_turbo_binding_resolves() {
    assert_eq!(qwen_router().resolve("turbo-node", None), QWEN_TURBO);
}

#[test]
fn qwen_long_binding_resolves() {
    assert_eq!(qwen_router().resolve("long-node", None), QWEN_LONG);
}

#[test]
fn qwen_vl_plus_binding_resolves() {
    assert_eq!(qwen_router().resolve("vl-node", None), QWEN_VL_PLUS);
}

#[test]
fn qwen_all_models_start_with_qwen() {
    for m in [QWEN3_MAX, QWEN3_PLUS, QWEN_TURBO, QWEN_LONG, QWEN_VL_PLUS] {
        assert!(m.starts_with("qwen"), "Expected qwen prefix in {m}");
    }
}

#[test]
fn qwen_all_models_distinct() {
    let models = [QWEN3_MAX, QWEN3_PLUS, QWEN_TURBO, QWEN_LONG, QWEN_VL_PLUS];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 5);
}

#[test]
fn qwen_bind_small_assigns_turbo() {
    let mut r = ModelRouter::new(QWEN3_MAX);
    r.bind_small(&["classify", "triage"], QWEN_TURBO);
    assert_eq!(r.resolve("classify", None), QWEN_TURBO);
    assert_eq!(r.resolve("plan", None), QWEN3_MAX);
}

#[test]
fn qwen_models_no_whitespace() {
    for m in [QWEN3_MAX, QWEN3_PLUS, QWEN_TURBO, QWEN_LONG, QWEN_VL_PLUS] {
        assert_eq!(m.trim(), m);
    }
}
