use crate::error::InferenceError;
use crate::types::{CompletionRequest, CompletionResponse};

/// Model-agnostic interface for sending completion requests.
pub trait ModelClient: Send + Sync {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError>;
}
