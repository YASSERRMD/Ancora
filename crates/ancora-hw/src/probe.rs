//! ancora-hw: Hardware probe -- detects cpu, gpu, npu, and memory at runtime.
//!
//! All detection is done via standard library and OS-level queries so that
//! the crate remains dependency-free beyond serde.

use crate::model::{CpuArch, GpuBackend, HardwareProfile, NpuPlatform};

/// Probe the current host and return a best-effort `HardwareProfile`.
///
/// The probe is entirely offline and non-destructive.  On platforms where
/// a particular capability cannot be determined, safe defaults are used.
pub fn probe_hardware() -> HardwareProfile {
    let cpu_arch = detect_cpu_arch();
    let (cpu_logical_cores, cpu_physical_cores) = detect_cpu_cores();
    let cpu_freq_mhz = detect_cpu_freq_mhz();
    let total_ram_mib = detect_total_ram_mib();
    let gpu_backend = detect_gpu_backend(&cpu_arch);
    let gpu_vram_mib = detect_gpu_vram_mib(&gpu_backend);
    let npu_platform = detect_npu_platform(&cpu_arch);
    let is_apple_silicon = is_apple_silicon_device(&cpu_arch);
    let has_arm_npu = detect_arm_npu(&cpu_arch, &npu_platform);

    HardwareProfile {
        cpu_arch,
        cpu_logical_cores,
        cpu_physical_cores,
        cpu_freq_mhz,
        total_ram_mib,
        gpu_backend,
        gpu_vram_mib,
        npu_platform,
        is_apple_silicon,
        has_arm_npu,
        thermal_pressure: 0,   // populated by thermal module
        power_budget_watts: 0, // populated by power module
    }
}

/// Detect the CPU architecture using Rust's compile-time target arch.
pub fn detect_cpu_arch() -> CpuArch {
    #[cfg(target_arch = "x86_64")]
    return CpuArch::X86_64;
    #[cfg(target_arch = "aarch64")]
    return CpuArch::Aarch64;
    #[allow(unreachable_code)]
    CpuArch::Unknown
}

/// Return (logical_cores, physical_cores) using available_parallelism().
pub fn detect_cpu_cores() -> (u32, u32) {
    let logical = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    // Physical cores: heuristic -- divide by 2 when HT is likely (x86-64),
    // keep same count for ARM (no SMT in most ARM chips).
    let physical = if cfg!(target_arch = "x86_64") {
        (logical / 2).max(1)
    } else {
        logical
    };
    (logical, physical)
}

/// Probe CPU base frequency in MHz.  Returns 0 when unavailable.
pub fn detect_cpu_freq_mhz() -> u32 {
    // On Linux, read /proc/cpuinfo for "cpu MHz".
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(val) = line.split(':').nth(1) {
                        if let Ok(f) = val.trim().parse::<f64>() {
                            return f as u32;
                        }
                    }
                }
            }
        }
    }
    0
}

/// Probe total system RAM in MiB.  Returns 512 as a conservative fallback.
pub fn detect_total_ram_mib() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return kb / 1024;
                        }
                    }
                }
            }
        }
    }
    // macOS: sysctl hw.memsize returns bytes as a string.
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Ok(bytes) = s.trim().parse::<u64>() {
                return bytes / (1024 * 1024);
            }
        }
    }
    512
}

/// Infer GPU backend from OS and architecture heuristics.
pub fn detect_gpu_backend(arch: &CpuArch) -> GpuBackend {
    #[cfg(target_os = "macos")]
    {
        // All Apple M-series and recent Intel Macs have Metal.
        let _ = arch;
        return GpuBackend::Metal;
    }
    #[cfg(target_os = "linux")]
    {
        // Check for nvidia device nodes.
        if std::path::Path::new("/dev/nvidia0").exists() {
            return GpuBackend::Cuda;
        }
        // Check for AMD render nodes.
        if std::path::Path::new("/dev/dri/renderD128").exists() {
            return GpuBackend::Rocm;
        }
    }
    let _ = arch;
    GpuBackend::None
}

/// Estimate GPU VRAM in MiB; 0 when no GPU backend is present.
pub fn detect_gpu_vram_mib(backend: &GpuBackend) -> u64 {
    match backend {
        GpuBackend::None => 0,
        GpuBackend::Metal => {
            // On Apple Silicon the GPU shares system memory; report 0 to
            // indicate the caller should use total_ram_mib instead.
            0
        }
        _ => 0, // VRAM detection requires native APIs beyond std
    }
}

/// Detect the NPU platform for this device.
pub fn detect_npu_platform(arch: &CpuArch) -> NpuPlatform {
    #[cfg(target_os = "macos")]
    {
        // All Apple M-series chips include the ANE.
        let _ = arch;
        return NpuPlatform::AppleAne;
    }
    match arch {
        CpuArch::Aarch64 => {
            // On Android/Linux ARM64, NNAPI may expose an NPU.
            #[cfg(target_os = "android")]
            return NpuPlatform::Nnapi;
            #[allow(unreachable_code)]
            NpuPlatform::None
        }
        _ => NpuPlatform::None,
    }
}

/// Returns true when the probe identifies this as an Apple Silicon device.
pub fn is_apple_silicon_device(arch: &CpuArch) -> bool {
    #[cfg(target_os = "macos")]
    {
        if matches!(arch, CpuArch::Aarch64) {
            return true;
        }
    }
    let _ = arch;
    false
}

/// Returns true when an ARM NPU is detected.
pub fn detect_arm_npu(arch: &CpuArch, npu: &NpuPlatform) -> bool {
    if !matches!(arch, CpuArch::Aarch64) {
        return false;
    }
    !matches!(npu, NpuPlatform::None)
}
