use crate::batch::{cpu_batch_hint, tune_batch_size, BatchConfig};
use crate::model::HardwareProfile;

fn hw_with_ram(ram_mib: u64) -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.total_ram_mib = ram_mib;
    hw.cpu_logical_cores = 8;
    hw
}

#[test]
fn batch_tuning_adjusts_to_memory() {
    let hw_small = hw_with_ram(2048);
    let hw_large = hw_with_ram(65536);

    let cfg = BatchConfig {
        model_footprint_mib: 512,
        ..BatchConfig::default()
    };

    let rec_small = tune_batch_size(&hw_small, &cfg);
    let rec_large = tune_batch_size(&hw_large, &cfg);

    // Larger RAM should yield a larger batch.
    assert!(rec_large.max_batch_size > rec_small.max_batch_size);
}

#[test]
fn batch_size_at_least_one() {
    let hw = hw_with_ram(256); // tiny device
    let cfg = BatchConfig {
        model_footprint_mib: 200,
        ..BatchConfig::default()
    };
    let rec = tune_batch_size(&hw, &cfg);
    assert!(rec.max_batch_size >= 1);
    assert!(rec.suggested_batch_size >= 1);
}

#[test]
fn cpu_batch_hint_matches_cores() {
    let mut hw = HardwareProfile::default_safe();
    hw.cpu_logical_cores = 16;
    assert_eq!(cpu_batch_hint(&hw), 16);
}
