/// Tests for the vLLM integration using mock transports.
use crate::model::{CompletionRequest, CompletionResult, EngineKind};
use crate::vllm::{SamplingParams, VllmClient, VllmError, VllmTransport};

struct OkTransport;

impl VllmTransport for OkTransport {
    fn complete(
        &self,
        _endpoint: &str,
        model: &str,
        prompt: &str,
        _params: &SamplingParams,
    ) -> Result<CompletionResult, VllmError> {
        Ok(CompletionResult {
            text: format!("{} says: {}", model, prompt),
            tokens_generated: 8,
            engine: EngineKind::Vllm,
        })
    }

    fn models(&self, _endpoint: &str) -> Result<Vec<String>, VllmError> {
        Ok(vec!["mistral-7b".to_string(), "llama3-8b".to_string()])
    }

    fn health(&self, _endpoint: &str) -> Result<bool, VllmError> {
        Ok(true)
    }
}

struct OomTransport;

impl VllmTransport for OomTransport {
    fn complete(
        &self,
        _endpoint: &str,
        _model: &str,
        _prompt: &str,
        _params: &SamplingParams,
    ) -> Result<CompletionResult, VllmError> {
        Err(VllmError::Oom)
    }

    fn models(&self, _endpoint: &str) -> Result<Vec<String>, VllmError> {
        Ok(vec![])
    }

    fn health(&self, _endpoint: &str) -> Result<bool, VllmError> {
        Ok(false)
    }
}

#[test]
fn vllm_complete_ok() {
    let config = crate::vllm::default_config();
    let client = VllmClient::new(config, "mistral-7b", OkTransport);
    let req = CompletionRequest::new("hello vllm");
    let res = client.complete(&req).unwrap();
    assert!(res.text.contains("mistral-7b"));
    assert!(res.text.contains("hello vllm"));
    assert_eq!(res.engine, EngineKind::Vllm);
}

#[test]
fn vllm_list_models_ok() {
    let config = crate::vllm::default_config();
    let client = VllmClient::new(config, "mistral-7b", OkTransport);
    let models = client.list_models().unwrap();
    assert!(models.contains(&"mistral-7b".to_string()));
}

#[test]
fn vllm_oom_error() {
    let config = crate::vllm::default_config();
    let client = VllmClient::new(config, "llama3", OomTransport);
    let req = CompletionRequest::new("test");
    let err = client.complete(&req).unwrap_err();
    assert!(matches!(err, VllmError::Oom));
}

#[test]
fn vllm_sampling_params_from_request() {
    let req = CompletionRequest::new("x")
        .with_max_tokens(512)
        .with_temperature(0.2);
    let params = SamplingParams::from_request(&req);
    assert_eq!(params.max_tokens, 512);
    assert!((params.temperature - 0.2).abs() < 1e-6);
}
