use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuotaError {
    #[error("hard rate limit exceeded for tenant {tenant}; retry after {retry_after_secs}s")]
    HardLimitExceeded { tenant: String, retry_after_secs: u64 },
    #[error("soft rate limit warning for tenant {tenant}: {metric} at {pct:.0}% of limit")]
    SoftLimitWarning { tenant: String, metric: String, pct: f64 },
    #[error("provider {provider} rate limit exceeded for tenant {tenant}; retry after {retry_after_secs}s")]
    ProviderLimitExceeded { tenant: String, provider: String, retry_after_secs: u64 },
}

impl QuotaError {
    pub fn is_blocking(&self) -> bool {
        matches!(self, QuotaError::HardLimitExceeded { .. } | QuotaError::ProviderLimitExceeded { .. })
    }

    pub fn retry_after_secs(&self) -> Option<u64> {
        match self {
            QuotaError::HardLimitExceeded { retry_after_secs, .. } => Some(*retry_after_secs),
            QuotaError::ProviderLimitExceeded { retry_after_secs, .. } => Some(*retry_after_secs),
            QuotaError::SoftLimitWarning { .. } => None,
        }
    }
}
