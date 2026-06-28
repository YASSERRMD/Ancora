use std::collections::HashMap;
use crate::error::QuotaError;
use crate::window::SlidingWindow;

/// Per-provider, per-tenant rate coordination.
/// Enforces a maximum requests-per-minute per provider key.
#[derive(Default)]
pub struct ProviderRateCoordinator {
    /// (tenant, provider) -> sliding window
    windows: HashMap<(String, String), SlidingWindow>,
}

impl ProviderRateCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a call to `provider` from `tenant`. Returns error when `max_rps` is exceeded.
    pub fn check(&mut self, tenant: &str, provider: &str, max_rps: u64, now: u64) -> Result<(), QuotaError> {
        let key = (tenant.to_owned(), provider.to_owned());
        let window = self.windows.entry(key).or_insert_with(|| SlidingWindow::new(60, now));
        window.increment(now, 1);
        if window.count > max_rps {
            let retry = window.seconds_until_reset(now).max(1);
            return Err(QuotaError::ProviderLimitExceeded {
                tenant: tenant.to_owned(),
                provider: provider.to_owned(),
                retry_after_secs: retry,
            });
        }
        Ok(())
    }
}
