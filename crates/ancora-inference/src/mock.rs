use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::types::{CompletionRequest, CompletionResponse};

/// A test double that returns a fixed response for every request.
pub struct MockClient {
    pub response: String,
}

impl ModelClient for MockClient {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        Ok(CompletionResponse {
            content: self.response.clone(),
            tokens_in: request.messages.iter().map(|m| m.content.len() as u64).sum(),
            tokens_out: self.response.len() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    fn req(content: &str) -> CompletionRequest {
        CompletionRequest {
            model_id: "mock".to_string(),
            messages: vec![Message { role: "user".to_string(), content: content.to_string() }],
            max_tokens: None,
            temperature: None,
        }
    }

    #[test]
    fn mock_client_returns_fixed_response() {
        let client = MockClient { response: "hello".to_string() };
        let resp = client.complete(&req("world")).unwrap();
        assert_eq!(resp.content, "hello");
        assert_eq!(resp.tokens_out, 5);
    }
}
