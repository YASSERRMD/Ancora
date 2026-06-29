use crate::fit::{can_run, check_fit, FitRejectionReason, FitResult};
use crate::model::{HardwareProfile, ModelRequirements};

fn small_cpu_hw() -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.total_ram_mib = 2048;
    hw
}

#[test]
fn fit_check_accepts_small_model() {
    let hw = small_cpu_hw();
    let req = ModelRequirements::cpu_only("tiny", 512, 7);
    assert!(can_run(&hw, &req));
}

#[test]
fn fit_check_rejects_too_large_model() {
    let hw = small_cpu_hw();
    // Model needs more RAM than the device has.
    let req = ModelRequirements::cpu_only("huge", 8192, 70_000);
    let result = check_fit(&hw, &req);
    assert_eq!(
        result,
        FitResult::DoesNotFit(FitRejectionReason::InsufficientRam)
    );
}

#[test]
fn fit_returns_cpu_when_no_gpu() {
    let hw = small_cpu_hw();
    let req = ModelRequirements::cpu_only("medium", 1024, 13);
    let result = check_fit(&hw, &req);
    assert_eq!(result, FitResult::Fits(crate::model::ComputeUnit::Cpu));
}

#[test]
fn fit_returns_npu_when_available_and_capable() {
    use crate::model::{CpuArch, GpuBackend, NpuPlatform};
    let hw = HardwareProfile {
        cpu_arch: CpuArch::Aarch64,
        cpu_logical_cores: 8,
        cpu_physical_cores: 8,
        cpu_freq_mhz: 3200,
        total_ram_mib: 16384,
        gpu_backend: GpuBackend::Metal,
        gpu_vram_mib: 0,
        npu_platform: NpuPlatform::AppleAne,
        is_apple_silicon: true,
        has_arm_npu: true,
        thermal_pressure: 0,
        power_budget_watts: 30,
    };
    let req = ModelRequirements {
        name: "ane-model".to_owned(),
        min_ram_mib: 1024,
        min_vram_mib: 0,
        npu_capable: true,
        params_millions: 7,
    };
    let result = check_fit(&hw, &req);
    assert_eq!(result, FitResult::Fits(crate::model::ComputeUnit::Npu));
}
