//! ancora-hw: Thermal and power awareness hooks.
//!
//! Provides a thermal probe, power budget estimation, and hooks that callers
//! can register to react to thermal/power state changes.

use crate::model::HardwareProfile;
use serde::{Deserialize, Serialize};

/// Thermal pressure levels matching Apple's `thermalState` enum and Linux
/// thermal zone conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThermalPressure {
    /// Device is running cool; full performance available.
    Nominal = 0,
    /// Moderate thermal load; minor throttling may begin.
    Fair = 1,
    /// Significant thermal load; throttling is active.
    Serious = 2,
    /// Device is critically hot; aggressive throttling.
    Critical = 3,
}

impl ThermalPressure {
    /// Convert from a raw u8 pressure level (clamped to 0-3).
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => ThermalPressure::Nominal,
            1 => ThermalPressure::Fair,
            2 => ThermalPressure::Serious,
            _ => ThermalPressure::Critical,
        }
    }

    /// Returns a throughput scaling factor (0.0 – 1.0) the scheduler should
    /// apply when this thermal pressure level is active.
    pub fn throughput_scale(&self) -> f64 {
        match self {
            ThermalPressure::Nominal => 1.0,
            ThermalPressure::Fair => 0.85,
            ThermalPressure::Serious => 0.60,
            ThermalPressure::Critical => 0.30,
        }
    }
}

/// Power budget configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerBudget {
    /// Maximum sustained power draw in watts.
    pub max_watts: u32,
    /// Current estimated power draw in watts.
    pub current_watts: u32,
}

impl PowerBudget {
    /// Returns the fraction of power budget currently consumed (0.0 – 1.0).
    pub fn utilization(&self) -> f64 {
        if self.max_watts == 0 {
            return 0.0;
        }
        (self.current_watts as f64 / self.max_watts as f64).min(1.0)
    }

    /// Returns true if the device is over 90 % of its power budget.
    pub fn is_constrained(&self) -> bool {
        self.utilization() >= 0.9
    }
}

/// Read the thermal pressure from the hardware profile.
pub fn read_thermal_pressure(hw: &HardwareProfile) -> ThermalPressure {
    ThermalPressure::from_u8(hw.thermal_pressure)
}

/// Estimate whether scheduling should be throttled given current conditions.
///
/// Returns a suggested concurrency scale factor.
pub fn thermal_concurrency_scale(hw: &HardwareProfile) -> f64 {
    read_thermal_pressure(hw).throughput_scale()
}

/// A simple thermal hook that records a handler reaction.
///
/// In a real runtime the scheduler would call registered hooks when the
/// thermal state changes.  This type captures the result of one such call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalHookResult {
    pub pressure: u8,
    pub action_taken: String,
}

/// Execute a thermal hook with the given pressure level.
///
/// The hook function receives the pressure level and returns a description of
/// the action it took.
pub fn run_thermal_hook<F>(pressure: ThermalPressure, hook: F) -> ThermalHookResult
where
    F: Fn(ThermalPressure) -> String,
{
    ThermalHookResult {
        pressure: pressure as u8,
        action_taken: hook(pressure),
    }
}
