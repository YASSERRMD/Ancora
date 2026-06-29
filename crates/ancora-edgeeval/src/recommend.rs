//! Model recommendation by device profile.
//!
//! Matches models to device profiles based on memory, compute, and power constraints.

use crate::model::SmallModel;

/// Device profile describing hardware constraints.
#[derive(Debug, Clone)]
pub struct DeviceProfile {
    pub name: String,
    /// Available RAM in MiB.
    pub ram_mib: u64,
    /// Max sustained compute in giga-operations per second (GOPS, INT8).
    pub compute_gops: f64,
    /// Battery capacity in mWh (0 = mains-powered).
    pub battery_mwh: f64,
    /// Max acceptable latency per token in ms.
    pub max_latency_per_token_ms: f64,
}

impl DeviceProfile {
    pub fn new(
        name: impl Into<String>,
        ram_mib: u64,
        compute_gops: f64,
        battery_mwh: f64,
        max_latency_per_token_ms: f64,
    ) -> Self {
        Self {
            name: name.into(),
            ram_mib,
            compute_gops,
            battery_mwh,
            max_latency_per_token_ms,
        }
    }

    /// Is this a battery-constrained device?
    pub fn is_battery_powered(&self) -> bool {
        self.battery_mwh > 0.0
    }

    /// Predefined profile: microcontroller (very constrained).
    pub fn microcontroller() -> Self {
        Self::new("microcontroller", 256, 0.5, 500.0, 500.0)
    }

    /// Predefined profile: mobile phone.
    pub fn mobile() -> Self {
        Self::new("mobile", 4096, 20.0, 4000.0, 100.0)
    }

    /// Predefined profile: laptop.
    pub fn laptop() -> Self {
        Self::new("laptop", 16384, 100.0, 50000.0, 50.0)
    }

    /// Predefined profile: edge server (mains-powered).
    pub fn edge_server() -> Self {
        Self::new("edge-server", 65536, 1000.0, 0.0, 10.0)
    }
}

/// A scored candidate model for a device.
#[derive(Debug, Clone)]
pub struct ModelCandidate {
    pub model: SmallModel,
    /// Estimated memory usage in MiB for this model.
    pub estimated_memory_mib: f64,
    /// Estimated latency per token in ms on this device.
    pub estimated_latency_ms: f64,
}

impl ModelCandidate {
    pub fn new(model: SmallModel, estimated_memory_mib: f64, estimated_latency_ms: f64) -> Self {
        Self {
            model,
            estimated_memory_mib,
            estimated_latency_ms,
        }
    }

    /// Estimate memory usage from model params and quantization bits.
    pub fn estimate_memory_mib(model: &SmallModel) -> f64 {
        let bytes_per_param = model.quantization_bits as f64 / 8.0;
        let total_bytes = model.param_count_millions as f64 * 1_000_000.0 * bytes_per_param;
        // Add 25% overhead for KV cache and activations.
        total_bytes * 1.25 / (1024.0 * 1024.0)
    }

    /// Estimate latency per token given device compute (GOPS).
    pub fn estimate_latency_ms(model: &SmallModel, device_gops: f64) -> f64 {
        if device_gops == 0.0 {
            return f64::INFINITY;
        }
        // Rough heuristic: ~2 ops per param per token, scaled by quantization.
        let ops_per_token = model.param_count_millions as f64 * 2.0 * 1e6; // ops
        let gops = device_gops * 1e9; // ops/s
        (ops_per_token / gops) * 1000.0 // ms
    }
}

/// Recommendation result.
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub device_name: String,
    pub recommended_model_name: Option<String>,
    pub reason: String,
    pub candidates_evaluated: usize,
}

impl Recommendation {
    pub fn no_match(device_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            device_name: device_name.into(),
            recommended_model_name: None,
            reason: reason.into(),
            candidates_evaluated: 0,
        }
    }
}

/// Recommender that matches models to device profiles.
#[derive(Debug, Default)]
pub struct DeviceRecommender {
    candidates: Vec<ModelCandidate>,
}

impl DeviceRecommender {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_candidate(&mut self, candidate: ModelCandidate) {
        self.candidates.push(candidate);
    }

    /// Recommend a model for the given device profile.
    pub fn recommend(&self, device: &DeviceProfile) -> Recommendation {
        let eligible: Vec<&ModelCandidate> = self
            .candidates
            .iter()
            .filter(|c| {
                // Must fit in RAM.
                let mem_ok = c.estimated_memory_mib <= device.ram_mib as f64;
                // Must meet latency requirement.
                let lat_ok = c.estimated_latency_ms <= device.max_latency_per_token_ms;
                mem_ok && lat_ok
            })
            .collect();

        if eligible.is_empty() {
            return Recommendation::no_match(
                device.name.clone(),
                "no candidate fits memory and latency constraints",
            );
        }

        // Pick the largest model (by params) that fits, as a quality proxy.
        let best = eligible
            .iter()
            .max_by_key(|c| c.model.param_count_millions)
            .unwrap();

        Recommendation {
            device_name: device.name.clone(),
            recommended_model_name: Some(best.model.name.clone()),
            reason: format!(
                "largest model fitting {:.0}MiB RAM and {:.0}ms latency constraint",
                device.ram_mib,
                device.max_latency_per_token_ms,
            ),
            candidates_evaluated: eligible.len(),
        }
    }
}
