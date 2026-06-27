/// Google Gemini provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const GEMINI_25_PRO: &str = "gemini-2-5-pro";
const GEMINI_25_FLASH: &str = "gemini-2-5-flash";
const GEMINI_15_PRO: &str = "gemini-1.5-pro";

fn gemini_router() -> ModelRouter {
    let mut r = ModelRouter::new(GEMINI_25_PRO);
    r.bind("flash-node", GEMINI_25_FLASH);
    r.bind("legacy-node", GEMINI_15_PRO);
    r
}

#[test]
fn gemini_default_is_25_pro() {
    let r = ModelRouter::new(GEMINI_25_PRO);
    assert_eq!(r.resolve("any", None), GEMINI_25_PRO);
}

#[test]
fn gemini_flash_binding_resolves() {
    assert_eq!(gemini_router().resolve("flash-node", None), GEMINI_25_FLASH);
}

#[test]
fn gemini_15_pro_binding_resolves() {
    assert_eq!(gemini_router().resolve("legacy-node", None), GEMINI_15_PRO);
}

#[test]
fn gemini_unbound_falls_back_to_25_pro() {
    assert_eq!(gemini_router().resolve("unbound", None), GEMINI_25_PRO);
}

#[test]
fn gemini_model_strings_start_with_gemini() {
    for m in [GEMINI_25_PRO, GEMINI_25_FLASH, GEMINI_15_PRO] {
        assert!(m.starts_with("gemini-"), "Expected gemini- prefix in {m}");
    }
}

#[test]
fn gemini_all_models_distinct() {
    let models = [GEMINI_25_PRO, GEMINI_25_FLASH, GEMINI_15_PRO];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn gemini_flash_assigned_to_cheap_nodes() {
    let mut r = ModelRouter::new(GEMINI_25_PRO);
    r.bind_small(&["summarize", "classify"], GEMINI_25_FLASH);
    assert_eq!(r.resolve("summarize", None), GEMINI_25_FLASH);
    assert_eq!(r.resolve("plan", None), GEMINI_25_PRO);
}

#[test]
fn gemini_models_no_whitespace() {
    for m in [GEMINI_25_PRO, GEMINI_25_FLASH, GEMINI_15_PRO] {
        assert_eq!(m.trim(), m);
    }
}

#[test]
fn gemini_rebind_takes_latest() {
    let mut r = ModelRouter::new(GEMINI_25_PRO);
    r.bind("n", GEMINI_15_PRO);
    r.bind("n", GEMINI_25_FLASH);
    assert_eq!(r.resolve("n", None), GEMINI_25_FLASH);
}
