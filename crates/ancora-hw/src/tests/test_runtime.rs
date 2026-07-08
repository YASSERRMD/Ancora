use crate::apple::build_apple_silicon_profile;
use crate::fit::FitResult;
use crate::model::{HardwareProfile, ModelRequirements};
use crate::runtime::{measure_schedule_overhead_ns, schedule};

fn base_hw() -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.cpu_logical_cores = 8;
    hw.total_ram_mib = 16384;
    hw
}

fn small_req() -> ModelRequirements {
    ModelRequirements::cpu_only("small-lm", 2048, 7)
}

#[test]
fn scheduling_deterministic_given_a_profile() {
    let hw = base_hw();
    let req = small_req();

    let d1 = schedule(&hw, &req, 32, 64);
    let d2 = schedule(&hw, &req, 32, 64);

    // Same inputs must produce identical outputs.
    assert_eq!(d1.batch.max_batch_size, d2.batch.max_batch_size);
    assert_eq!(d1.concurrency.limit, d2.concurrency.limit);
    assert_eq!(d1.offload.gpu_layers, d2.offload.gpu_layers);
    assert_eq!(d1.thermal_scale, d2.thermal_scale);
}

#[test]
fn scheduling_fits_for_adequate_hardware() {
    let hw = base_hw();
    let req = small_req();
    let d = schedule(&hw, &req, 32, 64);
    assert!(matches!(d.fit, FitResult::Fits(_)));
}

#[test]
fn scheduling_does_not_fit_when_ram_insufficient() {
    let mut hw = HardwareProfile::default_safe();
    hw.total_ram_mib = 256;
    let req = ModelRequirements::cpu_only("big", 8192, 70_000);
    let d = schedule(&hw, &req, 32, 256);
    assert!(matches!(d.fit, FitResult::DoesNotFit(_)));
}

#[test]
fn scheduling_overhead_measured() {
    let hw = base_hw();
    let req = small_req();
    let ns = measure_schedule_overhead_ns(&hw, &req, 32, 64);
    // Scheduling must complete in under 100 ms (100_000_000 ns).
    assert!(
        ns < 100_000_000,
        "scheduling took {}ns, expected < 100ms",
        ns
    );
}

#[test]
fn apple_silicon_schedule_uses_metal() {
    use crate::fit::FitResult;
    use crate::model::ComputeUnit;
    let hw = build_apple_silicon_profile(10, 32768);
    let req = ModelRequirements {
        name: "gpu-lm".to_owned(),
        min_ram_mib: 4096,
        min_vram_mib: 4096,
        npu_capable: false,
        params_millions: 7,
    };
    let d = schedule(&hw, &req, 32, 128);
    // On Apple Silicon with Metal, should fit on GPU.
    assert_eq!(d.fit, FitResult::Fits(ComputeUnit::Gpu));
}
