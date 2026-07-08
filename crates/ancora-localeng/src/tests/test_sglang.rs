/// Tests for the SGLang integration using mock transports.
use crate::model::{CompletionRequest, CompletionResult, EngineKind};
use crate::sglang::{SglangClient, SglangError, SglangStats, SglangTransport};

struct OkTransport;

impl SglangTransport for OkTransport {
    fn generate(
        &self,
        _endpoint: &str,
        model: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, SglangError> {
        Ok(CompletionResult {
            text: format!("sglang[{}]: {}", model, request.prompt),
            tokens_generated: 6,
            engine: EngineKind::Sglang,
        })
    }

    fn stats(&self, _endpoint: &str) -> Result<SglangStats, SglangError> {
        Ok(SglangStats {
            num_requests_running: 3,
            num_requests_waiting: 1,
            gpu_utilization: 0.72,
        })
    }

    fn health(&self, _endpoint: &str) -> Result<bool, SglangError> {
        Ok(true)
    }
}

struct ErrTransport;

impl SglangTransport for ErrTransport {
    fn generate(
        &self,
        _endpoint: &str,
        _model: &str,
        _request: &CompletionRequest,
    ) -> Result<CompletionResult, SglangError> {
        Err(SglangError::BadRequest("too long".to_string()))
    }

    fn stats(&self, _endpoint: &str) -> Result<SglangStats, SglangError> {
        Err(SglangError::Unreachable("mock".to_string()))
    }

    fn health(&self, _endpoint: &str) -> Result<bool, SglangError> {
        Ok(false)
    }
}

#[test]
fn sglang_complete_ok() {
    let config = crate::sglang::default_config();
    let client = SglangClient::new(config, "llama3", OkTransport);
    let req = CompletionRequest::new("test input");
    let res = client.complete(&req).unwrap();
    assert!(res.text.contains("llama3"));
    assert_eq!(res.engine, EngineKind::Sglang);
}

#[test]
fn sglang_stats_ok() {
    let config = crate::sglang::default_config();
    let client = SglangClient::new(config, "llama3", OkTransport);
    let stats = client.stats().unwrap();
    assert_eq!(stats.num_requests_running, 3);
    assert!((stats.gpu_utilization - 0.72).abs() < 1e-5);
}

#[test]
fn sglang_bad_request_error() {
    let config = crate::sglang::default_config();
    let client = SglangClient::new(config, "llama3", ErrTransport);
    let req = CompletionRequest::new("too long prompt here");
    let err = client.complete(&req).unwrap_err();
    assert!(matches!(err, SglangError::BadRequest(_)));
}
