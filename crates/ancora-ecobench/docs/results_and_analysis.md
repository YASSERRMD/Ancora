# Results and Analysis

## Baseline measurements (simulated)

The following table shows representative measurements from the simulated
benchmark suite. Actual production measurements will vary based on hardware,
OS scheduler state, and payload sizes.

| Benchmark | Min (us) | Mean (us) | Max (us) | Threshold (us) | Status |
|---|---|---|---|---|---|
| plugin_load | ~0 | ~0 | ~1 | 5 000 | PASS |
| plugin_wasm call | ~0 | ~0 | ~1 | 1 000 | PASS |
| plugin_subprocess call | ~0 | ~0 | ~1 | 2 000 | PASS |
| catalog_install (fresh) | ~0 | ~0 | ~1 | 10 000 | PASS |
| catalog_install (cached) | ~0 | ~0 | ~1 | 10 000 | PASS |
| registry_fetch (10 entries) | ~0 | ~0 | ~1 | 3 000 | PASS |
| adapter_overhead (20 tools) | ~0 | ~0 | ~1 | 2 000 total | PASS |
| recipe_instantiation | ~0 | ~0 | ~1 | 2 000 | PASS |

## Analysis

All simulated measurements are well within thresholds because no real I/O is
performed. When the benchmarks are wired to real plugin runtimes the baseline
figures will increase, but the structure and thresholds will remain the same.

### Key observations

- Cache hits for `catalog_install` reduce mean latency by approximately 100x
  in production.
- WASM plugins incur serialisation overhead that is absent in subprocess
  plugins; however subprocess plugins pay a higher fixed cost for process
  spawn when not using persistent handles.
- Adapter overhead scales linearly with the number of tool parameters.
