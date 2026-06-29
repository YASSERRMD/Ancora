use crate::model::{GpuBackend, HardwareProfile};
use crate::offload::{compute_offload_policy, layer_assignments, LayerTarget, OffloadPolicy};

fn cpu_only_hw() -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.total_ram_mib = 8192;
    hw
}

fn gpu_hw(vram_mib: u64) -> HardwareProfile {
    let mut hw = HardwareProfile::default_safe();
    hw.total_ram_mib = 16384;
    hw.gpu_backend = GpuBackend::Cuda;
    hw.gpu_vram_mib = vram_mib;
    hw
}

#[test]
fn offload_policy_cpu_only_when_no_gpu() {
    let hw = cpu_only_hw();
    let policy = compute_offload_policy(&hw, 32, 100);
    assert_eq!(policy.gpu_layers, 0);
    assert_eq!(policy.cpu_layers, 32);
}

#[test]
fn offload_policy_applied_correctly() {
    let hw = gpu_hw(8192);
    let policy = compute_offload_policy(&hw, 32, 200); // 200 MiB/layer, 8192/200=40 => all 32 layers fit
    assert_eq!(policy.gpu_layers, 32);
    assert_eq!(policy.cpu_layers, 0);
}

#[test]
fn layer_assignments_sum_to_total() {
    let policy = OffloadPolicy {
        total_layers: 20,
        gpu_layers: 10,
        npu_layers: 0,
        cpu_layers: 10,
    };
    let assignments = layer_assignments(&policy);
    assert_eq!(assignments.len(), 20);
    let gpu_count = assignments.iter().filter(|t| **t == LayerTarget::Gpu).count();
    let cpu_count = assignments.iter().filter(|t| **t == LayerTarget::Cpu).count();
    assert_eq!(gpu_count, 10);
    assert_eq!(cpu_count, 10);
}

#[test]
fn offload_policy_cpu_only_constructor() {
    let p = OffloadPolicy::cpu_only(16);
    assert_eq!(p.cpu_layers, 16);
    assert_eq!(p.gpu_layers, 0);
}

#[test]
fn offload_policy_gpu_only_constructor() {
    let p = OffloadPolicy::gpu_only(16);
    assert_eq!(p.gpu_layers, 16);
    assert_eq!(p.cpu_layers, 0);
}
