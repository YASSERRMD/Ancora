//! Resource cgroup limits for the headless agent service.
//!
//! Provides data structures and helpers for defining and applying
//! Linux cgroup v2 resource limits (CPU, memory, I/O) for the Ancora agent.

use std::collections::HashMap;

/// A cgroup resource limit specification.
#[derive(Debug, Clone)]
pub struct CgroupLimits {
    /// Name of the cgroup slice (e.g., "ancora.slice").
    pub slice: String,
    /// CPU quota as a percentage (1-100).
    pub cpu_quota_percent: u8,
    /// Memory limit in megabytes.
    pub memory_limit_mb: u64,
    /// Memory swap limit in megabytes (0 = no swap).
    pub memory_swap_mb: u64,
    /// IO weight (1-10000, default 100).
    pub io_weight: u32,
    /// OOM kill enabled flag.
    pub oom_kill_disable: bool,
    /// Additional arbitrary properties.
    pub extra: HashMap<String, String>,
}

impl Default for CgroupLimits {
    fn default() -> Self {
        CgroupLimits {
            slice: "ancora.slice".to_string(),
            cpu_quota_percent: 80,
            memory_limit_mb: 4096,
            memory_swap_mb: 0,
            io_weight: 100,
            oom_kill_disable: false,
            extra: HashMap::new(),
        }
    }
}

impl CgroupLimits {
    pub fn new(slice: impl Into<String>) -> Self {
        CgroupLimits {
            slice: slice.into(),
            ..Default::default()
        }
    }

    pub fn cpu_quota_percent(mut self, pct: u8) -> Self {
        self.cpu_quota_percent = pct.min(100);
        self
    }

    pub fn memory_limit_mb(mut self, mb: u64) -> Self {
        self.memory_limit_mb = mb;
        self
    }

    pub fn memory_swap_mb(mut self, mb: u64) -> Self {
        self.memory_swap_mb = mb;
        self
    }

    pub fn io_weight(mut self, weight: u32) -> Self {
        self.io_weight = weight.clamp(1, 10000);
        self
    }

    /// Converts CPU quota percentage to a cgroup v2 cpu.max value string.
    /// Format: "MAX PERIOD" where period is 100000 microseconds.
    pub fn cpu_max_value(&self) -> String {
        let period = 100_000u64;
        let quota = (period * self.cpu_quota_percent as u64) / 100;
        format!("{} {}", quota, period)
    }

    /// Returns the memory limit in bytes as a string.
    pub fn memory_max_value(&self) -> String {
        format!("{}", self.memory_limit_mb * 1024 * 1024)
    }

    /// Returns the swap+memory limit in bytes as a string.
    pub fn memory_swap_value(&self) -> String {
        if self.memory_swap_mb == 0 {
            "0".to_string()
        } else {
            format!(
                "{}",
                (self.memory_limit_mb + self.memory_swap_mb) * 1024 * 1024
            )
        }
    }

    /// Checks whether the limits are within a safe range for headless use.
    pub fn validate(&self) -> Result<(), String> {
        if self.cpu_quota_percent == 0 {
            return Err("cpu_quota_percent must be > 0".to_string());
        }
        if self.memory_limit_mb < 64 {
            return Err("memory_limit_mb must be >= 64".to_string());
        }
        Ok(())
    }
}

/// The result of applying cgroup limits to the running service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CgroupApplyResult {
    Applied,
    AlreadyApplied,
    NotSupported(String),
    Error(String),
}

impl std::fmt::Display for CgroupApplyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CgroupApplyResult::Applied => write!(f, "applied"),
            CgroupApplyResult::AlreadyApplied => write!(f, "already-applied"),
            CgroupApplyResult::NotSupported(r) => write!(f, "not-supported: {}", r),
            CgroupApplyResult::Error(e) => write!(f, "error: {}", e),
        }
    }
}

/// Simulates applying cgroup limits (in real usage would write to /sys/fs/cgroup/).
pub fn apply_limits(limits: &CgroupLimits) -> CgroupApplyResult {
    match limits.validate() {
        Ok(_) => CgroupApplyResult::Applied,
        Err(e) => CgroupApplyResult::Error(e),
    }
}

/// Snapshot of current cgroup resource usage.
#[derive(Debug, Clone)]
pub struct CgroupUsage {
    pub cpu_usage_percent: f64,
    pub memory_used_mb: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

impl CgroupUsage {
    pub fn zero() -> Self {
        CgroupUsage {
            cpu_usage_percent: 0.0,
            memory_used_mb: 0,
            io_read_bytes: 0,
            io_write_bytes: 0,
        }
    }

    /// Returns true if memory usage exceeds the given limit.
    pub fn exceeds_memory(&self, limit: &CgroupLimits) -> bool {
        self.memory_used_mb > limit.memory_limit_mb
    }

    /// Returns true if CPU usage exceeds the configured quota.
    pub fn exceeds_cpu(&self, limit: &CgroupLimits) -> bool {
        self.cpu_usage_percent > limit.cpu_quota_percent as f64
    }
}

/// Preset cgroup limits for different deployment sizes.
pub mod presets {
    use super::CgroupLimits;

    /// Minimal preset for very constrained inference OS (RPi-class hardware).
    pub fn minimal() -> CgroupLimits {
        CgroupLimits::new("ancora-minimal.slice")
            .cpu_quota_percent(50)
            .memory_limit_mb(512)
            .io_weight(50)
    }

    /// Standard preset for a dedicated inference node.
    pub fn standard() -> CgroupLimits {
        CgroupLimits::new("ancora.slice")
            .cpu_quota_percent(80)
            .memory_limit_mb(4096)
            .io_weight(100)
    }

    /// Full preset for a high-memory inference server.
    pub fn full() -> CgroupLimits {
        CgroupLimits::new("ancora-full.slice")
            .cpu_quota_percent(95)
            .memory_limit_mb(32768)
            .io_weight(200)
    }
}
