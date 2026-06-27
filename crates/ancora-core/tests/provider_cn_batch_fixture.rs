/// Chinese provider batch: StepFun, Ernie, HunyuanDouBao, Mimo -- offline, no HTTP calls.
use ancora_core::routing::ModelRouter;

// StepFun
const STEP_2: &str       = "step-2-16k";
const STEP_1_V: &str     = "step-1v-32k";

// Baidu Ernie
const ERNIE_SPEED: &str  = "ernie-speed-128k";
const ERNIE_4: &str      = "ernie-4.0-8k";

// Tencent Hunyuan
const HUNYUAN_PRO: &str  = "hunyuan-pro";
const HUNYUAN_LITE: &str = "hunyuan-lite";

// Doubao (ByteDance)
const DOUBAO_PRO: &str   = "doubao-pro-32k";
const DOUBAO_LITE: &str  = "doubao-lite-4k";

// Mimo (placeholder)
const MIMO_7B: &str      = "mimo-7b";

fn cn_router() -> ModelRouter {
    let mut r = ModelRouter::new(ERNIE_4);
    r.bind("step2-node", STEP_2);
    r.bind("stepv-node", STEP_1_V);
    r.bind("ernie-speed-node", ERNIE_SPEED);
    r.bind("hunyuan-pro-node", HUNYUAN_PRO);
    r.bind("hunyuan-lite-node", HUNYUAN_LITE);
    r.bind("doubao-pro-node", DOUBAO_PRO);
    r.bind("doubao-lite-node", DOUBAO_LITE);
    r.bind("mimo-node", MIMO_7B);
    r
}

#[test]
fn stepfun_step2_binding_resolves() {
    assert_eq!(cn_router().resolve("step2-node", None), STEP_2);
}

#[test]
fn stepfun_step1v_binding_resolves() {
    assert_eq!(cn_router().resolve("stepv-node", None), STEP_1_V);
}

#[test]
fn ernie_speed_binding_resolves() {
    assert_eq!(cn_router().resolve("ernie-speed-node", None), ERNIE_SPEED);
}

#[test]
fn ernie_default_is_ernie4() {
    assert_eq!(cn_router().resolve("unbound", None), ERNIE_4);
}

#[test]
fn hunyuan_pro_binding_resolves() {
    assert_eq!(cn_router().resolve("hunyuan-pro-node", None), HUNYUAN_PRO);
}

#[test]
fn hunyuan_lite_binding_resolves() {
    assert_eq!(cn_router().resolve("hunyuan-lite-node", None), HUNYUAN_LITE);
}

#[test]
fn doubao_pro_binding_resolves() {
    assert_eq!(cn_router().resolve("doubao-pro-node", None), DOUBAO_PRO);
}

#[test]
fn doubao_lite_binding_resolves() {
    assert_eq!(cn_router().resolve("doubao-lite-node", None), DOUBAO_LITE);
}

#[test]
fn mimo_7b_binding_resolves() {
    assert_eq!(cn_router().resolve("mimo-node", None), MIMO_7B);
}

#[test]
fn all_cn_models_distinct() {
    let models = [
        STEP_2, STEP_1_V, ERNIE_SPEED, ERNIE_4,
        HUNYUAN_PRO, HUNYUAN_LITE, DOUBAO_PRO, DOUBAO_LITE, MIMO_7B,
    ];
    let unique: std::collections::HashSet<&str> = models.iter().copied().collect();
    assert_eq!(unique.len(), 9);
}

#[test]
fn all_cn_models_no_whitespace() {
    for m in [STEP_2, STEP_1_V, ERNIE_SPEED, ERNIE_4, HUNYUAN_PRO, HUNYUAN_LITE, DOUBAO_PRO, DOUBAO_LITE, MIMO_7B] {
        assert_eq!(m.trim(), m);
    }
}
