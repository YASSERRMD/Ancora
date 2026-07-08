#[cfg(test)]
mod tests {
    use crate::QuotaError;

    #[test]
    fn hard_limit_is_blocking() {
        let err = QuotaError::HardLimitExceeded {
            tenant: "t".into(),
            retry_after_secs: 30,
        };
        assert!(err.is_blocking());
        assert_eq!(err.retry_after_secs(), Some(30));
    }

    #[test]
    fn soft_limit_is_non_blocking() {
        let err = QuotaError::SoftLimitWarning {
            tenant: "t".into(),
            metric: "requests".into(),
            pct: 85.0,
        };
        assert!(!err.is_blocking());
        assert_eq!(err.retry_after_secs(), None);
    }

    #[test]
    fn provider_limit_is_blocking_with_retry_after() {
        let err = QuotaError::ProviderLimitExceeded {
            tenant: "t".into(),
            provider: "openai".into(),
            retry_after_secs: 60,
        };
        assert!(err.is_blocking());
        assert_eq!(err.retry_after_secs(), Some(60));
    }
}
