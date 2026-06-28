use crate::resource_limits::{ResourceLimits, ResourceViolation};

#[test]
fn plugin_cannot_exceed_cpu_limit() {
    let limits = ResourceLimits {
        max_cpu_ms: Some(500),
        max_memory_bytes: None,
        max_threads: None,
        max_open_fds: None,
    };
    // Within limit - should pass.
    assert!(limits.check(499, 0, 0, 0).is_ok());
    // Exactly at limit - should pass.
    assert!(limits.check(500, 0, 0, 0).is_ok());
    // Over limit - must fail.
    let err = limits.check(501, 0, 0, 0).unwrap_err();
    assert!(matches!(err, ResourceViolation::CpuExceeded { .. }));
}

#[test]
fn plugin_cannot_exceed_memory_limit() {
    let limits = ResourceLimits {
        max_cpu_ms: None,
        max_memory_bytes: Some(1024),
        max_threads: None,
        max_open_fds: None,
    };
    assert!(limits.check(0, 1024, 0, 0).is_ok());
    let err = limits.check(0, 1025, 0, 0).unwrap_err();
    assert!(matches!(err, ResourceViolation::MemoryExceeded { .. }));
}

#[test]
fn plugin_cannot_exceed_thread_limit() {
    let limits = ResourceLimits {
        max_cpu_ms: None,
        max_memory_bytes: None,
        max_threads: Some(2),
        max_open_fds: None,
    };
    assert!(limits.check(0, 0, 2, 0).is_ok());
    let err = limits.check(0, 0, 3, 0).unwrap_err();
    assert!(matches!(err, ResourceViolation::ThreadsExceeded { .. }));
}

#[test]
fn plugin_cannot_exceed_fd_limit() {
    let limits = ResourceLimits {
        max_cpu_ms: None,
        max_memory_bytes: None,
        max_threads: None,
        max_open_fds: Some(8),
    };
    assert!(limits.check(0, 0, 0, 8).is_ok());
    let err = limits.check(0, 0, 0, 9).unwrap_err();
    assert!(matches!(err, ResourceViolation::FdsExceeded { .. }));
}

#[test]
fn unlimited_permits_any_usage() {
    let limits = ResourceLimits::unlimited();
    assert!(limits.check(u64::MAX, u64::MAX, u32::MAX, u32::MAX).is_ok());
}
