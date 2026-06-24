/// A single turn in a conversation, identified by role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// A request sent to a model for completion.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model_id: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

/// The response returned by a model after completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionResponse {
    pub content: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
}
