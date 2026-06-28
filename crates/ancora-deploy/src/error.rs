use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeployError {
    #[error("incompatible worker versions: {a} and {b} have different major versions")]
    IncompatibleVersion { a: String, b: String },
    #[error("canary failed health gate: error rate {error_rate:.1}% exceeds threshold {threshold:.1}%")]
    CanaryHealthGateFailed { error_rate: f64, threshold: f64 },
    #[error("drain incomplete: {active_runs} runs still active")]
    DrainIncomplete { active_runs: u32 },
    #[error("rollback failed: {0}")]
    RollbackFailed(String),
}
