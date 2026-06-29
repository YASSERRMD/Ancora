# Device Profiles

ancora-hw categorises devices into profiles to drive scheduling decisions.

## Apple Silicon (M-series)

Detected via `CpuArch::Aarch64` on macOS.  All M-series devices have:

- Metal GPU backend sharing unified system memory.
- Apple Neural Engine (ANE) for NPU-capable models.
- No discrete VRAM -- GPU memory budget = `total_ram_mib * gpu_memory_fraction`.

| Tier      | Logical cores | GPU command queues | ANE TOPS |
|-----------|---------------|--------------------|----------|
| MBase     | ≤ 8           | 2                  | 11–18    |
| MPro      | 10–11         | 3                  | 11–18    |
| MMax      | 12–14         | 4                  | 18–38    |
| MUltra    | ≥ 24          | 8                  | 38+      |

## x86-64 Server / Workstation

Detected via `CpuArch::X86_64`.

- CUDA or ROCm GPU backend when device nodes are present.
- GPU VRAM is measured separately from system RAM.
- No NPU in most configurations.

## ARM Edge Device

Detected via `CpuArch::Aarch64` on Linux / Android.

- Qualcomm HTP or NNAPI-backed NPU may be present.
- Power budget is often very constrained (< 5 W TDP).

## Fallback / Unknown

When hardware cannot be detected, ancora-hw uses:

- 1 logical core, 512 MiB RAM.
- No GPU, no NPU.
- Thermal pressure = Nominal.
- Concurrency limit = 1.
