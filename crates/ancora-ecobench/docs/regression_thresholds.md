# Regression Thresholds

## Purpose

Regression thresholds define the maximum acceptable mean latency for each
benchmark. Any commit that causes a mean to exceed its threshold will fail
the CI `ecobench` job.

## Threshold values

These are defined as constants in each module and also recorded in benchmark
result files via `result_schema::BenchRecord::threshold_ns`.

| Module | Constant | Value |
|---|---|---|
| `plugin_load` | `LOAD_TARGET_US` | 5 000 us |
| `plugin_wasm` | `WASM_CALL_TARGET_US` | 1 000 us |
| `plugin_subprocess` | `SUBPROCESS_CALL_TARGET_US` | 2 000 us |
| `catalog_install` | `INSTALL_TARGET_US` | 10 000 us |
| `registry_fetch` | `FETCH_TARGET_US` | 3 000 us |
| `builder_export` | `EXPORT_TARGET_US` | 5 000 us |
| `adapter_overhead` | `ADAPT_PER_TOOL_TARGET_US` | 100 us/tool |
| `recipe_instantiation` | `INSTANTIATE_TARGET_US` | 2 000 us |

## Adjustment process

Thresholds should only be raised when:

1. A deliberate architectural change is accepted that increases latency.
2. The PR includes a documented justification in this file.
3. The adjusted value is still within an acceptable user-facing SLO.

Thresholds must never be raised speculatively to make a failing benchmark
pass without a corresponding code change.
