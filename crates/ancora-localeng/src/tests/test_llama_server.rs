/// Tests for the llama.cpp server integration using mock transports.

use crate::llama_server::{LlamaServerClient, LlamaServerError, LlamaServerTransport};
use crate::model::{CompletionRequest, CompletionResult, EngineKind};

/// Mock transport that always succeeds.
struct OkTransport {
    response: String,
}

impl LlamaServerTransport for OkTransport {
    fn post_completion(
        &self,
        _endpoint: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, LlamaServerError> {
        Ok(CompletionResult {
            text: format!("{} -> {}", request.prompt, self.response),
            tokens_generated: self.response.split_whitespace().count(),
            engine: EngineKind::LlamaCppServer,
        })
    }

    fn get_health(&self, _endpoint: &str) -> Result<bool, LlamaServerError> {
        Ok(true)
    }
}

/// Mock transport that always fails.
struct ErrTransport;

impl LlamaServerTransport for ErrTransport {
    fn post_completion(
        &self,
        _endpoint: &str,
        _request: &CompletionRequest,
    ) -> Result<CompletionResult, LlamaServerError> {
        Err(LlamaServerError::Unreachable("mock".to_string()))
    }

    fn get_health(&self, _endpoint: &str) -> Result<bool, LlamaServerError> {
        Err(LlamaServerError::Unreachable("mock".to_string()))
    }
}

#[test]
fn llama_server_complete_ok() {
    let config = crate::llama_server::default_config();
    let transport = OkTransport {
        response: "world".to_string(),
    };
    let client = LlamaServerClient::new(config, transport);
    let req = CompletionRequest::new("hello");
    let result = client.complete(&req).expect("should succeed");
    assert!(result.text.contains("hello"));
    assert!(result.text.contains("world"));
    assert_eq!(result.engine, EngineKind::LlamaCppServer);
}

#[test]
fn llama_server_health_ok() {
    let config = crate::llama_server::default_config();
    let client = LlamaServerClient::new(config, OkTransport { response: String::new() });
    assert!(client.health().unwrap());
}

#[test]
fn llama_server_complete_err() {
    let config = crate::llama_server::default_config();
    let client = LlamaServerClient::new(config, ErrTransport);
    let req = CompletionRequest::new("hello");
    assert!(client.complete(&req).is_err());
}

#[test]
fn llama_server_endpoint_uses_config() {
    let config = crate::llama_server::default_config()
        .with_endpoint("http://10.0.0.1:9090");
    let client = LlamaServerClient::new(config, ErrTransport);
    assert_eq!(client.endpoint(), "http://10.0.0.1:9090");
}
