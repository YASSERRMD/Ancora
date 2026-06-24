/// Errors returned by inference adapter implementations.
#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("model refused: {0}")]
    Refused(String),
    #[error("http error {status}: {body}")]
    Http { status: u16, body: String },
    #[error("parse error: {0}")]
    Parse(String),
    #[error("endpoint unreachable: {0}")]
    Unreachable(String),
    #[error("internal error: {0}")]
    Internal(String),
}
