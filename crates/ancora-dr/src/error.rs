use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrError {
    #[error("replication lag {lag} exceeds maximum allowed {max_allowed_lag}")]
    LagTooHigh { lag: u64, max_allowed_lag: u64 },
    #[error("not in failover state; cannot failback")]
    NotInFailoverState,
    #[error("primary is fenced and cannot accept writes")]
    PrimaryFenced,
}
