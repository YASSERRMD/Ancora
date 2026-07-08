/// Tests for the Ollama integration using mock transports.
use crate::model::{CompletionRequest, CompletionResult, EngineKind};
use crate::ollama::{OllamaClient, OllamaError, OllamaModelList, OllamaTransport};

struct OkTransport {
    models: Vec<String>,
}

impl OllamaTransport for OkTransport {
    fn list_models(&self, _endpoint: &str) -> Result<OllamaModelList, OllamaError> {
        Ok(OllamaModelList {
            models: self.models.clone(),
        })
    }

    fn generate(
        &self,
        _endpoint: &str,
        model: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, OllamaError> {
        Ok(CompletionResult {
            text: format!("[{}] {}", model, request.prompt),
            tokens_generated: 5,
            engine: EngineKind::Ollama,
        })
    }

    fn ping(&self, _endpoint: &str) -> Result<bool, OllamaError> {
        Ok(true)
    }
}

struct ErrTransport;

impl OllamaTransport for ErrTransport {
    fn list_models(&self, _endpoint: &str) -> Result<OllamaModelList, OllamaError> {
        Err(OllamaError::Unreachable("mock".to_string()))
    }

    fn generate(
        &self,
        _endpoint: &str,
        _model: &str,
        _request: &CompletionRequest,
    ) -> Result<CompletionResult, OllamaError> {
        Err(OllamaError::ModelNotFound("llama3".to_string()))
    }

    fn ping(&self, _endpoint: &str) -> Result<bool, OllamaError> {
        Err(OllamaError::Unreachable("mock".to_string()))
    }
}

#[test]
fn ollama_list_models_ok() {
    let config = crate::ollama::default_config();
    let t = OkTransport {
        models: vec!["llama3".to_string(), "mistral".to_string()],
    };
    let client = OllamaClient::new(config, "llama3", t);
    let list = client.list_models().unwrap();
    assert!(list.contains("llama3"));
    assert!(list.contains("mistral"));
    assert!(!list.contains("gpt4"));
}

#[test]
fn ollama_complete_ok() {
    let config = crate::ollama::default_config();
    let client = OllamaClient::new(config, "llama3", OkTransport { models: vec![] });
    let req = CompletionRequest::new("ping");
    let res = client.complete(&req).unwrap();
    assert!(res.text.contains("llama3"));
    assert_eq!(res.engine, EngineKind::Ollama);
}

#[test]
fn ollama_model_not_found_error() {
    let config = crate::ollama::default_config();
    let client = OllamaClient::new(config, "llama3", ErrTransport);
    let req = CompletionRequest::new("ping");
    let err = client.complete(&req).unwrap_err();
    assert!(matches!(err, OllamaError::ModelNotFound(_)));
}

#[test]
fn ollama_ping_ok() {
    let config = crate::ollama::default_config();
    let client = OllamaClient::new(config, "llama3", OkTransport { models: vec![] });
    assert!(client.ping().unwrap());
}
