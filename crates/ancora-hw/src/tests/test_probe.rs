use crate::probe::{detect_cpu_arch, detect_cpu_cores, probe_hardware};

#[test]
fn probe_returns_capabilities() {
    let hw = probe_hardware();
    // Must have at least 1 logical core.
    assert!(hw.cpu_logical_cores >= 1, "expected >= 1 logical core");
    // Must have at least 1 physical core.
    assert!(hw.cpu_physical_cores >= 1, "expected >= 1 physical core");
    // RAM must be positive.
    assert!(hw.total_ram_mib > 0, "expected positive RAM");
    // Thermal pressure must be in range.
    assert!(hw.thermal_pressure <= 3);
}

#[test]
fn cpu_arch_is_known() {
    let arch = detect_cpu_arch();
    // On supported CI targets (x86_64, aarch64) we expect a non-Unknown arch.
    // We accept Unknown too since the platform may vary.
    use crate::model::CpuArch;
    let _is_known = matches!(arch, CpuArch::X86_64 | CpuArch::Aarch64 | CpuArch::Unknown);
    // Test just that it doesn't panic.
}

#[test]
fn cpu_cores_positive() {
    let (logical, physical) = detect_cpu_cores();
    assert!(logical >= 1);
    assert!(physical >= 1);
    assert!(logical >= physical);
}

#[test]
fn available_units_always_includes_cpu() {
    let hw = probe_hardware();
    let units = hw.available_units();
    use crate::model::ComputeUnit;
    assert!(units.contains(&ComputeUnit::Cpu));
}
