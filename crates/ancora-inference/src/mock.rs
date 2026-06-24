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
}
