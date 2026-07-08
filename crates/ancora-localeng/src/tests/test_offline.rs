/// Offline integration tests validating that all mocked engine paths
/// execute correctly without any network access.
use crate::capability::Capabilities;
use crate::health::{HealthChecker, MockHealthChecker};
use crate::llama_embedded::{EmbeddedEngine, MockEmbeddedBackend};
use crate::model::{CompletionRequest, EngineConfig, EngineKind};
use crate::runtime::{select_engine, HardwareProfile, SelectionCriteria};

#[test]
fn offline_llama_embedded_round_trip() {
    let config =
        EngineConfig::new(EngineKind::LlamaCppEmbedded).with_model_path("/fake/llama.gguf");
    let backend = MockEmbeddedBackend::new().with_fixed_response("42");
    let mut engine = EmbeddedEngine::new(config, backend);
    engine.load().expect("mock load should succeed");
    assert!(engine.is_ready());
    let req = CompletionRequest::new("what is the answer?");
    let res = engine.complete(&req).expect("mock infer should succeed");
    assert!(res.text.contains("42"));
}

#[test]
fn offline_health_all_engines() {
    let engines = [
        EngineKind::LlamaCppServer,
        EngineKind::LlamaCppEmbedded,
        EngineKind::Ollama,
        EngineKind::Vllm,
        EngineKind::Sglang,
        EngineKind::LmStudio,
        EngineKind::Tgi,
        EngineKind::OnnxRuntime,
    ];
    for engine in &engines {
        let checker = MockHealthChecker::healthy(engine.clone());
        let status = checker.check();
        assert!(status.is_ready(), "engine {} should report healthy", engine);
    }
}

#[test]
fn offline_capability_all_engines_have_known_caps() {
    let engines = [
        EngineKind::LlamaCppServer,
        EngineKind::LlamaCppEmbedded,
        EngineKind::Ollama,
        EngineKind::Vllm,
        EngineKind::Sglang,
        EngineKind::LmStudio,
        EngineKind::Tgi,
        EngineKind::OnnxRuntime,
    ];
    for engine in &engines {
        let _caps = Capabilities::for_engine(engine);
        // No panic = pass
    }
}

#[test]
fn offline_engine_selection_cpu_low_ram() {
    let hw = HardwareProfile::cpu_only(8.0, 4);
    let criteria = SelectionCriteria::new(hw);
    let result = select_engine(&criteria);
    assert_eq!(result.engine, EngineKind::LlamaCppEmbedded);
}

#[test]
fn offline_engine_selection_cpu_high_ram() {
    let hw = HardwareProfile::cpu_only(32.0, 8);
    let criteria = SelectionCriteria::new(hw);
    let result = select_engine(&criteria);
    assert_eq!(result.engine, EngineKind::LlamaCppServer);
}

#[test]
fn offline_engine_selection_gpu_throughput() {
    let hw = HardwareProfile::cpu_only(64.0, 16).with_cuda(24.0);
    let criteria = SelectionCriteria::new(hw).prefer_throughput();
    let result = select_engine(&criteria);
    assert_eq!(result.engine, EngineKind::Vllm);
}

#[test]
fn offline_engine_selection_gpu_ease() {
    let hw = HardwareProfile::cpu_only(32.0, 8).with_metal(8.0);
    let criteria = SelectionCriteria::new(hw);
    let result = select_engine(&criteria);
    assert_eq!(result.engine, EngineKind::Ollama);
}
