use crate::error::InferenceError;
use crate::types::{CompletionRequest, CompletionResponse, TokenEvent};

/// Model-agnostic interface for sending completion requests.
pub trait ModelClient: Send + Sync {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError>;

    /// Stream token fragments to `on_token` as they arrive.
    ///
    /// The default implementation calls `complete` and emits the full content as a single token.
    fn stream_complete(
        &self,
        request: &CompletionRequest,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<CompletionResponse, InferenceError> {
        let resp = self.complete(request)?;
        on_token(TokenEvent { text: resp.content.clone(), finished: true });
        Ok(resp)
    }
}
