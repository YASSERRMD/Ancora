/// Tests for the ONNX Runtime integration.
use crate::model::{CompletionRequest, EngineKind};
use crate::onnx::{
    ExecutionProvider, MockOnnxSession, OnnxEngine, OnnxError, OnnxSession, OnnxSessionConfig,
};

struct AlwaysFailSession;

impl OnnxSession for AlwaysFailSession {
    fn run(&self, _input_ids: &[i64]) -> Result<Vec<f32>, OnnxError> {
        Err(OnnxError::InferenceFailed("always fails".to_string()))
    }

    fn input_names(&self) -> Vec<String> {
        vec!["input_ids".to_string()]
    }

    fn output_names(&self) -> Vec<String> {
        vec!["logits".to_string()]
    }
}

#[test]
fn onnx_mock_session_run_ok() {
    let session = MockOnnxSession::new("/fake/model.onnx");
    let input_ids: Vec<i64> = vec![1, 2, 3, 4];
    let logits = session.run(&input_ids).unwrap();
    assert_eq!(logits.len(), 1000);
    assert!((logits[0] - 0.1).abs() < 1e-6);
}

#[test]
fn onnx_mock_session_empty_input_fails() {
    let session = MockOnnxSession::new("/fake/model.onnx");
    let err = session.run(&[]).unwrap_err();
    assert!(matches!(err, OnnxError::InvalidInput(_)));
}

#[test]
fn onnx_engine_complete_ok() {
    let config = crate::onnx::default_config();
    let sc = OnnxSessionConfig::new("/fake/model.onnx");
    let session = MockOnnxSession::new("/fake/model.onnx");
    let engine = OnnxEngine::new(config, sc, session);
    let req = CompletionRequest::new("the quick brown fox");
    let res = engine.complete(&req).unwrap();
    assert_eq!(res.engine, EngineKind::OnnxRuntime);
    assert!(!res.text.is_empty());
}

#[test]
fn onnx_engine_fail_session() {
    let config = crate::onnx::default_config();
    let sc = OnnxSessionConfig::new("/fake/model.onnx");
    let engine = OnnxEngine::new(config, sc, AlwaysFailSession);
    let req = CompletionRequest::new("hello");
    assert!(engine.complete(&req).is_err());
}

#[test]
fn onnx_session_config_providers() {
    let sc = OnnxSessionConfig::new("/model.onnx")
        .with_providers(vec![ExecutionProvider::Cuda, ExecutionProvider::Cpu]);
    assert_eq!(sc.providers[0], ExecutionProvider::Cuda);
    assert_eq!(sc.providers[1], ExecutionProvider::Cpu);
}

#[test]
fn execution_provider_display() {
    assert_eq!(ExecutionProvider::CoreML.to_string(), "CoreML");
    assert_eq!(ExecutionProvider::Cuda.to_string(), "CUDA");
    assert_eq!(ExecutionProvider::Cpu.to_string(), "CPU");
}
