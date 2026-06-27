/// GLM (Zhipu AI) provider mapping fixture -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

const GLM5: &str       = "glm-5";
const GLM4_PLUS: &str  = "glm-4-plus";
const GLM4: &str       = "glm-4";
const GLM4_AIR: &str   = "glm-4-air";
const GLM4_FLASH: &str = "glm-4-flash";

fn glm_router() -> ModelRouter {
    let mut r = ModelRouter::new(GLM5);
    r.bind("4plus-node", GLM4_PLUS);
    r.bind("4-node", GLM4);
    r.bind("air-node", GLM4_AIR);
    r.bind("flash-node", GLM4_FLASH);
    r
}

#[test]
fn glm_default_is_glm5() {
    assert_eq!(glm_router().resolve("any", None), GLM5);
}

#[test]
fn glm4_plus_binding_resolves() {
    assert_eq!(glm_router().resolve("4plus-node", None), GLM4_PLUS);
}

#[test]
fn glm4_binding_resolves() {
    assert_eq!(glm_router().resolve("4-node", None), GLM4);
}

#[test]
fn glm4_air_binding_resolves() {
    assert_eq!(glm_router().resolve("air-node", None), GLM4_AIR);
}

#[test]
fn glm4_flash_binding_resolves() {
    assert_eq!(glm_router().resolve("flash-node", None), GLM4_FLASH);
}

#[test]
fn glm_all_models_start_with_glm() {
    for m in [GLM5, GLM4_PLUS, GLM4, GLM4_AIR, GLM4_FLASH] {
        assert!(m.starts_with("glm-"), "Expected glm- prefix in {m}");
    }
}

#[test]
fn glm_all_models_distinct() {
    let models = [GLM5, GLM4_PLUS, GLM4, GLM4_AIR, GLM4_FLASH];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 5);
}

#[test]
fn glm_bind_small_assigns_flash() {
    let mut r = ModelRouter::new(GLM5);
    r.bind_small(&["fast-1", "fast-2"], GLM4_FLASH);
    assert_eq!(r.resolve("fast-1", None), GLM4_FLASH);
    assert_eq!(r.resolve("complex", None), GLM5);
}

#[test]
fn glm_models_no_whitespace() {
    for m in [GLM5, GLM4_PLUS, GLM4, GLM4_AIR, GLM4_FLASH] {
        assert_eq!(m.trim(), m);
    }
}
