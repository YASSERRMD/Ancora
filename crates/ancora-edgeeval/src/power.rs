//! Power-proxy metric for edge model evaluation.
//!
//! Re-exports `PowerProxy` from the runtime module and adds helpers for
//! thermal and battery-life estimation on constrained devices.

pub use crate::runtime::PowerProxy;

/// A thermal envelope constraint for edge devices.
#[derive(Debug, Clone)]
pub struct ThermalEnvelope {
    pub device_name: String,
    /// Maximum sustained power draw in milliwatts.
    pub max_power_mw: f64,
}

impl ThermalEnvelope {
    pub fn new(device_name: impl Into<String>, max_power_mw: f64) -> Self {
        Self {
            device_name: device_name.into(),
            max_power_mw,
        }
    }

    /// Compute the maximum token throughput (tokens/s) before hitting thermal limit.
    /// power_proxy.mwh_per_1k_tokens is converted to mW (mWh/1k_tokens * 3600 s/hr = mW at that rate).
    pub fn max_tokens_per_second(&self, proxy: &PowerProxy) -> f64 {
        if proxy.mwh_per_1k_tokens == 0.0 {
            return f64::INFINITY;
        }
        // mWh/1k_tokens * 3600 s/hr = mW * 1 s / token-batch
        // power_mw = (tps / 1000) * mwh_per_1k_tokens * 3600
        // tps_max = max_power_mw * 1000 / (mwh_per_1k_tokens * 3600)
        self.max_power_mw * 1000.0 / (proxy.mwh_per_1k_tokens * 3600.0)
    }

    /// Check whether a given token rate stays within thermal budget.
    pub fn is_sustainable(&self, proxy: &PowerProxy, tokens_per_second: f64) -> bool {
        tokens_per_second <= self.max_tokens_per_second(proxy)
    }
}

/// Rate a list of power proxies and return the most efficient one (highest tokens/joule).
pub fn most_efficient<'a>(proxies: &'a [(String, PowerProxy)]) -> Option<&'a str> {
    proxies
        .iter()
        .max_by(|(_, a), (_, b)| {
            a.tokens_per_joule()
                .partial_cmp(&b.tokens_per_joule())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(name, _)| name.as_str())
}
