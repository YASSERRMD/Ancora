//! ancora-hw: Hardware-aware scheduling for the Ancora agent framework.
//!
//! Provides:
//! - Hardware probe (cpu, gpu, npu, memory)
//! - Model-to-hardware fit check
//! - Batch size tuning
//! - Offload policy (cpu/gpu split)
//! - Thermal and power awareness hooks
//! - Concurrency limit calculation
//! - Apple Silicon detection and tuning
//! - ARM NPU detection and scheduling hints
//! - Deterministic scheduling given a hardware profile

pub mod apple;
pub mod armnpu;
pub mod batch;
pub mod concurrency;
pub mod config;
pub mod fit;
pub mod model;
pub mod offload;
pub mod probe;
pub mod runtime;
pub mod thermal;

#[cfg(test)]
mod tests;

// Re-exports for ergonomic use.
pub use apple::{apple_silicon_tuning, build_apple_silicon_profile, detect_apple_silicon_tier};
pub use armnpu::{arm_npu_scheduling_hints, detect_arm_npu_capability};
pub use batch::{tune_batch_size, BatchConfig, BatchRecommendation};
pub use concurrency::{
    compute_concurrency_limit, default_concurrency_limit, ConcurrencyConfig, ConcurrencyLimit,
};
pub use config::{parse_override, serialize_profile, HardwareProfileOverride};
pub use fit::{can_run, check_fit, FitResult};
pub use model::{
    ComputeUnit, CpuArch, GpuBackend, HardwareProfile, ModelRequirements, NpuPlatform,
};
pub use offload::{compute_offload_policy, layer_assignments, OffloadPolicy};
pub use probe::probe_hardware;
pub use runtime::{measure_schedule_overhead_ns, schedule, SchedulingDecision};
pub use thermal::{read_thermal_pressure, run_thermal_hook, ThermalPressure};
