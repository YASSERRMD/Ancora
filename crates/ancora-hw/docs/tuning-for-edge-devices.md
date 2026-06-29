# Tuning for Edge Devices

Edge devices (phones, embedded boards, IoT gateways) have tight memory,
power, and thermal constraints.  ancora-hw provides several knobs to tune
scheduling for these environments.

## Recommended settings

### Override detected hardware

When runtime detection is unreliable (e.g., behind a container), supply a
JSON override:

```json
{
  "cpu_logical_cores": 4,
  "total_ram_mib": 2048,
  "thermal_pressure": 1,
  "power_budget_watts": 5
}
```

```rust
use ancora_hw::{parse_override, probe_hardware};
let json = std::fs::read_to_string("hw_override.json").unwrap();
let ov = parse_override(&json).unwrap();
let hw = ov.apply(probe_hardware());
```

### Reduce headroom fraction

The default headroom factor is 0.85.  On memory-constrained devices, reduce
it to 0.70 to leave more room for the OS:

```rust
use ancora_hw::{tune_batch_size, BatchConfig, probe_hardware};
let hw = probe_hardware();
let cfg = BatchConfig {
    headroom: 0.70,
    model_footprint_mib: 512,
    ..BatchConfig::default()
};
let rec = tune_batch_size(&hw, &cfg);
```

### Reduce concurrency

On single-core or dual-core devices, set `core_fraction` to 1.0 but cap
`max_concurrency` at 2:

```rust
use ancora_hw::{compute_concurrency_limit, ConcurrencyConfig, probe_hardware};
let hw = probe_hardware();
let cfg = ConcurrencyConfig {
    core_fraction: 1.0,
    max_concurrency: 2,
    ..ConcurrencyConfig::default()
};
let limit = compute_concurrency_limit(&hw, &cfg);
```

### Respect thermal hooks

Register a thermal hook to reduce load dynamically:

```rust
use ancora_hw::{run_thermal_hook, ThermalPressure};
let result = run_thermal_hook(ThermalPressure::Serious, |_| {
    // pause new request intake
    "paused".to_owned()
});
```

## General advice

- Always use the fit check before loading a model.
- Prefer NPU offloading on Qualcomm and Apple Silicon to free CPU cores.
- Monitor `thermal_pressure` and back off when it reaches `Serious` (2).
- Use quantised (int4/int8) models on devices with < 4 GiB RAM.
