use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::types::{CompletionRequest, CompletionResponse};

/// A test double that returns a fixed response for every request.
pub struct MockClient {
    pub response: String,
}
