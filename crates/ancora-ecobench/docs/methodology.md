# Benchmark Methodology

## Overview

The `ancora-ecobench` benchmarks measure extension and packaging overhead for
the Ancora agent framework. All benchmarks are designed to be:

- **Deterministic**: no real network I/O; in-memory simulation of I/O boundaries.
- **Reproducible**: fixed iteration counts, warm-up phases, and sorted sample
  collections produce consistent statistics across runs.
- **Offline**: all tests pass without any network access.

## Measurement approach

Each benchmark domain is isolated in its own module. The shared `harness`
module runs each function under test for a configurable number of warm-up
and measurement iterations, then computes min, max, mean, and median.

### Phases measured

| Phase | Module | Threshold |
|---|---|---|
| Plugin load | `plugin_load` | 5 000 us |
| WASM call | `plugin_wasm` | 1 000 us |
| Subprocess call | `plugin_subprocess` | 2 000 us |
| Catalog install | `catalog_install` | 10 000 us |
| Registry fetch | `registry_fetch` | 3 000 us |
| Graph export | `builder_export` | 5 000 us |
| Adapter overhead | `adapter_overhead` | 100 us/tool |
| Recipe instantiation | `recipe_instantiation` | 2 000 us |

## Regression gating

The `result_schema` module serialises results to a key=value record. CI
compares `mean_ns` against `threshold_ns`; any exceedance fails the job.
