use ancora_proto::ancora::ErrorCode;

/// The unified error type for the Ancora core engine.
#[derive(Debug, thiserror::Error)]
pub enum AncoraError {
    // ---- Replay and determinism ----
    #[error("nondeterminism detected at seq {seq}: expected {expected}, got {got}")]
    Nondeterminism {
        seq: u64,
        expected: String,
        got: String,
    },
    #[error("journal gap at seq {seq}")]
    JournalGap { seq: u64 },
    #[error("journal write failed: {0}")]
    JournalWrite(String),

    // ---- Agent loop ----
    #[error("agent reached max_steps ({max_steps}) without producing output")]
    MaxSteps { max_steps: u32 },
    #[error("output validation failed after {attempts} repair attempt(s): {reason}")]
    OutputValidation { attempts: u32, reason: String },
    #[error("activity timed out after {timeout_ms} ms")]
    Timeout { timeout_ms: u64 },

    // ---- Inference ----
    #[error("model refused to respond: {0}")]
    ModelRefused(String),
    #[error("model HTTP error {status}: {body}")]
    ModelHttp { status: u16, body: String },
    #[error("model response could not be parsed: {0}")]
    ModelParse(String),
    #[error("model endpoint unreachable: {0}")]
    ModelUnreachable(String),

    // ---- Tools ----
    #[error("tool '{name}' returned an error: {message}")]
    ToolFailed { name: String, message: String },
    #[error("tool '{0}' is not registered")]
    ToolNotFound(String),
    #[error("tool '{name}' input invalid: {reason}")]
    ToolInputInvalid { name: String, reason: String },
    #[error("tool '{0}' call denied by permission broker")]
    ToolDenied(String),

    // ---- Policy ----
    #[error("residency policy blocked call to endpoint '{0}'")]
    PolicyResidency(String),
    #[error("required permission '{0}' was not granted")]
    PolicyPermission(String),

    // ---- Graph ----
    #[error("graph is invalid: {0}")]
    GraphInvalid(String),
    #[error("node '{0}' not found in graph")]
    NodeNotFound(String),

    // ---- Run lifecycle ----
    #[error("run was cancelled: {0}")]
    Cancelled(String),
    #[error("invalid state transition: {0}")]
    InvalidState(String),

    // ---- Storage ----
    #[error("storage error: {0}")]
    Storage(String),

    // ---- Internal ----
    #[error("internal error: {0}")]
    Internal(String),
}

impl AncoraError {
    /// Returns the canonical `ErrorCode` wire value for this error.
    pub fn error_code(&self) -> ErrorCode {
        match self {
            AncoraError::Nondeterminism { .. } => ErrorCode::ErrorNondeterminism,
            AncoraError::JournalGap { .. } => ErrorCode::ErrorJournalGap,
            AncoraError::JournalWrite(_) => ErrorCode::ErrorJournalWrite,
            AncoraError::MaxSteps { .. } => ErrorCode::ErrorMaxSteps,
            AncoraError::OutputValidation { .. } => ErrorCode::ErrorOutputValidation,
            AncoraError::Timeout { .. } => ErrorCode::ErrorTimeout,
            AncoraError::ModelRefused(_) => ErrorCode::ErrorModelRefused,
            AncoraError::ModelHttp { .. } => ErrorCode::ErrorModelHttp,
            AncoraError::ModelParse(_) => ErrorCode::ErrorModelParse,
            AncoraError::ModelUnreachable(_) => ErrorCode::ErrorModelUnreachable,
            AncoraError::ToolFailed { .. } => ErrorCode::ErrorToolFailed,
            AncoraError::ToolNotFound(_) => ErrorCode::ErrorToolNotFound,
            AncoraError::ToolInputInvalid { .. } => ErrorCode::ErrorToolInputInvalid,
            AncoraError::ToolDenied(_) => ErrorCode::ErrorToolDenied,
            AncoraError::PolicyResidency(_) => ErrorCode::ErrorPolicyResidency,
            AncoraError::PolicyPermission(_) => ErrorCode::ErrorPolicyPermission,
            AncoraError::GraphInvalid(_) => ErrorCode::ErrorGraphInvalid,
            AncoraError::NodeNotFound(_) => ErrorCode::ErrorNodeNotFound,
            AncoraError::Cancelled(_) => ErrorCode::ErrorCancelled,
            AncoraError::InvalidState(_) => ErrorCode::ErrorInvalidState,
            AncoraError::Storage(_) => ErrorCode::ErrorStorage,
            AncoraError::Internal(_) => ErrorCode::ErrorInternal,
        }
    }
}

/// Build an `AncoraError` from a wire `ErrorCode` and a message string.
/// Used when deserializing error events from the journal.
impl From<(ErrorCode, String)> for AncoraError {
    fn from((code, message): (ErrorCode, String)) -> Self {
        match code {
            ErrorCode::ErrorNondeterminism => AncoraError::Nondeterminism {
                seq: 0,
                expected: message.clone(),
                got: message,
            },
            ErrorCode::ErrorJournalGap => AncoraError::JournalGap { seq: 0 },
            ErrorCode::ErrorJournalWrite => AncoraError::JournalWrite(message),
            ErrorCode::ErrorMaxSteps => AncoraError::MaxSteps { max_steps: 0 },
            ErrorCode::ErrorOutputValidation => AncoraError::OutputValidation {
                attempts: 0,
                reason: message,
            },
            ErrorCode::ErrorTimeout => AncoraError::Timeout { timeout_ms: 0 },
            ErrorCode::ErrorModelRefused => AncoraError::ModelRefused(message),
            ErrorCode::ErrorModelHttp => AncoraError::ModelHttp {
                status: 0,
                body: message,
            },
            ErrorCode::ErrorModelParse => AncoraError::ModelParse(message),
            ErrorCode::ErrorModelUnreachable => AncoraError::ModelUnreachable(message),
            ErrorCode::ErrorToolFailed => AncoraError::ToolFailed {
                name: String::new(),
                message,
            },
            ErrorCode::ErrorToolNotFound => AncoraError::ToolNotFound(message),
            ErrorCode::ErrorToolInputInvalid => AncoraError::ToolInputInvalid {
                name: String::new(),
                reason: message,
            },
            ErrorCode::ErrorToolDenied => AncoraError::ToolDenied(message),
            ErrorCode::ErrorPolicyResidency => AncoraError::PolicyResidency(message),
            ErrorCode::ErrorPolicyPermission => AncoraError::PolicyPermission(message),
            ErrorCode::ErrorGraphInvalid => AncoraError::GraphInvalid(message),
            ErrorCode::ErrorNodeNotFound => AncoraError::NodeNotFound(message),
            ErrorCode::ErrorCancelled => AncoraError::Cancelled(message),
            ErrorCode::ErrorInvalidState => AncoraError::InvalidState(message),
            ErrorCode::ErrorStorage => AncoraError::Storage(message),
            ErrorCode::ErrorInternal | ErrorCode::ErrorUnspecified => {
                AncoraError::Internal(message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_non_unspecified_codes() -> Vec<ErrorCode> {
        vec![
            ErrorCode::ErrorNondeterminism,
            ErrorCode::ErrorJournalGap,
            ErrorCode::ErrorJournalWrite,
            ErrorCode::ErrorMaxSteps,
            ErrorCode::ErrorOutputValidation,
            ErrorCode::ErrorTimeout,
            ErrorCode::ErrorModelRefused,
            ErrorCode::ErrorModelHttp,
            ErrorCode::ErrorModelParse,
            ErrorCode::ErrorModelUnreachable,
            ErrorCode::ErrorToolFailed,
            ErrorCode::ErrorToolNotFound,
            ErrorCode::ErrorToolInputInvalid,
            ErrorCode::ErrorToolDenied,
            ErrorCode::ErrorPolicyResidency,
            ErrorCode::ErrorPolicyPermission,
            ErrorCode::ErrorGraphInvalid,
            ErrorCode::ErrorNodeNotFound,
            ErrorCode::ErrorCancelled,
            ErrorCode::ErrorInvalidState,
            ErrorCode::ErrorStorage,
            ErrorCode::ErrorInternal,
        ]
    }

    #[test]
    fn error_code_round_trips_for_all_non_unspecified_variants() {
        for code in all_non_unspecified_codes() {
            let err = AncoraError::from((code, "test".to_string()));
            assert_eq!(
                err.error_code() as i32,
                code as i32,
                "round-trip failed for code {:?}",
                code
            );
        }
    }

    #[test]
    fn error_unspecified_maps_to_internal() {
        let err = AncoraError::from((ErrorCode::ErrorUnspecified, "fallback".to_string()));
        assert!(
            matches!(err, AncoraError::Internal(_)),
            "ErrorUnspecified must map to AncoraError::Internal"
        );
    }

    #[test]
    fn error_messages_are_non_empty() {
        let err = AncoraError::Nondeterminism {
            seq: 7,
            expected: "foo".to_string(),
            got: "bar".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("7"), "display should include seq");
        assert!(msg.contains("foo"));
        assert!(msg.contains("bar"));
    }

    #[test]
    fn coverage_count_matches_proto() {
        assert_eq!(all_non_unspecified_codes().len(), 22);
    }
}
