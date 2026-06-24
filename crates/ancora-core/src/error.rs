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
