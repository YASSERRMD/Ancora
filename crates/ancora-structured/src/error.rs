use thiserror::Error;

#[derive(Debug, Error)]
pub enum StructuredError {
    #[error("response is not a JSON object")]
    NotAnObject,
    #[error("missing required field: {field}")]
    MissingField { field: String },
    #[error("type mismatch for field {field}: expected {expected}, got {got}")]
    TypeMismatch {
        field: String,
        expected: String,
        got: String,
    },
    #[error("JSON parse error: {0}")]
    ParseError(String),
    #[error("extraction failed: model did not produce structured output")]
    ExtractionFailed,
    #[error("retry limit reached after {attempts} attempts")]
    RetryLimitReached { attempts: u32 },
}
