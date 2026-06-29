use crate::concurrency::{compute_concurrency_limit, default_concurrency_limit, ConcurrencyConfig};
use crate::model::HardwareProfile;

fn hw(cores: u32, ram_mib: u64) -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.cpu_logical_cores = cores;
    hw.total_ram_mib = ram_mib;
    hw
}

#[test]
fn concurrency_limited_by_memory() {
    // 1 GiB RAM, 512 MiB per request => max 2 concurrent.
    let h = hw(64, 1024);
    let cfg = ConcurrencyConfig {
        mem_per_request_mib: 512,
        core_fraction: 1.0,
        min_concurrency: 1,
        max_concurrency: 256,
    };
    let limit = compute_concurrency_limit(&h, &cfg);
    assert!(limit.limit <= 2, "expected limit <= 2, got {}", limit.limit);
}

#[test]
fn concurrency_at_least_min() {
    let h = hw(1, 128); // tiny device
    let limit = default_concurrency_limit(&h);
    assert!(limit.limit >= 1);
}

#[test]
fn concurrency_does_not_exceed_max() {
    let h = hw(256, 1_000_000); // unrealistically large
    let cfg = ConcurrencyConfig {
        mem_per_request_mib: 1,
        core_fraction: 1.0,
        min_concurrency: 1,
        max_concurrency: 64,
    };
    let limit = compute_concurrency_limit(&h, &cfg);
    assert!(limit.limit <= 64);
}

#[test]
fn concurrency_limited_by_hardware_thermal() {
    let mut h = hw(16, 32768);
    h.thermal_pressure = 3; // critical -- scale = 0.30
    let cfg = ConcurrencyConfig {
        mem_per_request_mib: 128,
        core_fraction: 0.75,
        min_concurrency: 1,
        max_concurrency: 256,
    };
    let limit_hot = compute_concurrency_limit(&h, &cfg);

    h.thermal_pressure = 0; // nominal -- scale = 1.0
    let limit_cool = compute_concurrency_limit(&h, &cfg);

    assert!(limit_hot.limit <= limit_cool.limit);
}
