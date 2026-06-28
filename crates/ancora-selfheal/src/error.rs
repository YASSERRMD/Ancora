use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelfHealError {
    #[error("run stuck: {run_id} has not progressed for {elapsed_secs}s")]
    RunStuck { run_id: String, elapsed_secs: u64 },
    #[error("max requeue attempts reached for run: {run_id}")]
    MaxRequeueExceeded { run_id: String },
    #[error("circuit breaker open for: {name}")]
    CircuitOpen { name: String },
    #[error("no active provider available")]
    NoActiveProvider,
}
