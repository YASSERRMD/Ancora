# Hardware-Aware Scheduling in Ancora

ancora-hw schedules inference work to fit the physical device it runs on.
Rather than assuming a homogeneous server environment, the scheduler probes
available CPU cores, GPU VRAM, NPU presence, system RAM, and thermal state,
then derives a plan that maximises throughput without exceeding device limits.

## Core concepts

### Hardware probe

`probe_hardware()` returns a `HardwareProfile` describing the current device.
The probe is offline, non-destructive, and completes in microseconds.

```rust
use ancora_hw::probe_hardware;
let hw = probe_hardware();
println!("Logical cores: {}", hw.cpu_logical_cores);
println!("RAM: {} MiB", hw.total_ram_mib);
```

### Fit check

Before loading a model, verify it fits on the device:

```rust
use ancora_hw::{can_run, ModelRequirements, probe_hardware};
let hw = probe_hardware();
let req = ModelRequirements::cpu_only("my-model", 4096, 7);
if !can_run(&hw, &req) {
    eprintln!("Model does not fit on this device");
}
```

### Scheduling a workload

`schedule()` produces a complete `SchedulingDecision` deterministically:

```rust
use ancora_hw::{schedule, probe_hardware, ModelRequirements};
let hw = probe_hardware();
let req = ModelRequirements::cpu_only("my-model", 4096, 7);
let decision = schedule(&hw, &req, 32, 128);
println!("Batch: {}", decision.batch.suggested_batch_size);
println!("Concurrency: {}", decision.concurrency.limit);
```

## Design principles

1. **Determinism** -- given the same `HardwareProfile`, `schedule()` always
   returns the same result.
2. **Offline** -- no network calls, no external databases.
3. **No unsafe** -- pure safe Rust, no FFI for detection.
4. **Fail-safe defaults** -- unknown hardware falls back to conservative limits.
