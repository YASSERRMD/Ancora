use crate::cgroup::{apply_limits, presets, CgroupApplyResult, CgroupLimits, CgroupUsage};

#[test]
fn test_cgroup_limits_applied() {
    let limits = CgroupLimits::default();
    let result = apply_limits(&limits);
    assert_eq!(result, CgroupApplyResult::Applied);
}

#[test]
fn test_cgroup_cpu_max_value() {
    let limits = CgroupLimits::default().cpu_quota_percent(50);
    assert_eq!(limits.cpu_max_value(), "50000 100000");
}

#[test]
fn test_cgroup_memory_max_value() {
    let limits = CgroupLimits::default().memory_limit_mb(1024);
    assert_eq!(
        limits.memory_max_value(),
        format!("{}", 1024u64 * 1024 * 1024)
    );
}

#[test]
fn test_cgroup_validate_invalid_cpu() {
    let limits = CgroupLimits {
        cpu_quota_percent: 0,
        ..CgroupLimits::default()
    };
    assert!(limits.validate().is_err());
}

#[test]
fn test_cgroup_validate_invalid_memory() {
    let limits = CgroupLimits {
        memory_limit_mb: 16,
        ..CgroupLimits::default()
    };
    assert!(limits.validate().is_err());
}

#[test]
fn test_cgroup_usage_within_limits() {
    let limits = CgroupLimits::default();
    let usage = CgroupUsage {
        cpu_usage_percent: 50.0,
        memory_used_mb: 1024,
        io_read_bytes: 0,
        io_write_bytes: 0,
    };
    assert!(!usage.exceeds_memory(&limits));
    assert!(!usage.exceeds_cpu(&limits));
}

#[test]
fn test_cgroup_usage_exceeds_memory() {
    let limits = CgroupLimits::default().memory_limit_mb(512);
    let usage = CgroupUsage {
        cpu_usage_percent: 10.0,
        memory_used_mb: 1024,
        io_read_bytes: 0,
        io_write_bytes: 0,
    };
    assert!(usage.exceeds_memory(&limits));
}

#[test]
fn test_cgroup_preset_minimal() {
    let p = presets::minimal();
    assert!(p.memory_limit_mb <= 512);
}

#[test]
fn test_cgroup_preset_full() {
    let p = presets::full();
    assert!(p.memory_limit_mb >= 16384);
}
