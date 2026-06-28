/// Per-plugin resource limits.
///
/// All fields are optional (None = unlimited).  The host enforces limits by
/// polling the plugin's reported consumption against these thresholds and
/// terminating or rejecting operations that would exceed them.

/// CPU time budget expressed in milliseconds.
pub type CpuMillis = u64;

/// Memory budget in bytes.
pub type MemoryBytes = u64;

/// Maximum number of threads the plugin may spawn.
pub type ThreadCount = u32;

/// Maximum number of file descriptors the plugin may hold open simultaneously.
pub type FdCount = u32;

/// Aggregated resource limits for a plugin instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum CPU wall-clock time per invocation (milliseconds).
    pub max_cpu_ms: Option<CpuMillis>,
    /// Maximum resident memory (bytes).
    pub max_memory_bytes: Option<MemoryBytes>,
    /// Maximum threads the plugin may create.
    pub max_threads: Option<ThreadCount>,
    /// Maximum open file descriptors.
    pub max_open_fds: Option<FdCount>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_ms: Some(5_000),        // 5-second default wall-clock limit
            max_memory_bytes: Some(64 * 1024 * 1024), // 64 MiB default
            max_threads: Some(4),
            max_open_fds: Some(32),
        }
    }
}

impl ResourceLimits {
    /// No resource limits at all (unrestricted).
    pub fn unlimited() -> Self {
        Self {
            max_cpu_ms: None,
            max_memory_bytes: None,
            max_threads: None,
            max_open_fds: None,
        }
    }

    /// Returns `true` when the supplied usage values fit within the configured
    /// limits. A `None` limit means the corresponding resource is uncapped.
    pub fn check(
        &self,
        cpu_ms: CpuMillis,
        memory_bytes: MemoryBytes,
        threads: ThreadCount,
        open_fds: FdCount,
    ) -> Result<(), ResourceViolation> {
        if let Some(max) = self.max_cpu_ms {
            if cpu_ms > max {
                return Err(ResourceViolation::CpuExceeded { used: cpu_ms, limit: max });
            }
        }
        if let Some(max) = self.max_memory_bytes {
            if memory_bytes > max {
                return Err(ResourceViolation::MemoryExceeded { used: memory_bytes, limit: max });
            }
        }
        if let Some(max) = self.max_threads {
            if threads > max {
                return Err(ResourceViolation::ThreadsExceeded { used: threads, limit: max });
            }
        }
        if let Some(max) = self.max_open_fds {
            if open_fds > max {
                return Err(ResourceViolation::FdsExceeded { used: open_fds, limit: max });
            }
        }
        Ok(())
    }
}

/// Describes which resource constraint was violated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceViolation {
    CpuExceeded { used: CpuMillis, limit: CpuMillis },
    MemoryExceeded { used: MemoryBytes, limit: MemoryBytes },
    ThreadsExceeded { used: ThreadCount, limit: ThreadCount },
    FdsExceeded { used: FdCount, limit: FdCount },
}

impl std::fmt::Display for ResourceViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CpuExceeded { used, limit } =>
                write!(f, "CPU limit exceeded: used {}ms, limit {}ms", used, limit),
            Self::MemoryExceeded { used, limit } =>
                write!(f, "Memory limit exceeded: used {} bytes, limit {} bytes", used, limit),
            Self::ThreadsExceeded { used, limit } =>
                write!(f, "Thread limit exceeded: used {}, limit {}", used, limit),
            Self::FdsExceeded { used, limit } =>
                write!(f, "FD limit exceeded: used {}, limit {}", used, limit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_limits_allow_modest_usage() {
        let limits = ResourceLimits::default();
        assert!(limits.check(100, 1024 * 1024, 1, 4).is_ok());
    }

    #[test]
    fn cpu_violation_detected() {
        let limits = ResourceLimits { max_cpu_ms: Some(1000), ..ResourceLimits::unlimited() };
        let err = limits.check(2000, 0, 0, 0).unwrap_err();
        assert!(matches!(err, ResourceViolation::CpuExceeded { .. }));
    }

    #[test]
    fn memory_violation_detected() {
        let limits = ResourceLimits { max_memory_bytes: Some(1024), ..ResourceLimits::unlimited() };
        let err = limits.check(0, 2048, 0, 0).unwrap_err();
        assert!(matches!(err, ResourceViolation::MemoryExceeded { .. }));
    }
}
