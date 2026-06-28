# Ancora Configuration Reference

## Overview

Ancora uses a layered configuration system. Layers are merged in order:

```
base defaults < file overlay < env overlay < tenant overlay
```

Each layer overrides only the fields it specifies. Secrets are never stored inline; they are referenced by a `provider:key` string and resolved at use time.

## Top-level sections

### `core`

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `log_level` | string | `"info"` | One of: trace, debug, info, warn, error |
| `data_dir` | string | `"/var/ancora"` | Path to local data storage |
| `max_concurrent_runs` | u32 | `8` | Global cap on simultaneous agent runs |

### `journal`

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `flush_interval_ms` | u64 | `500` | Journal flush cadence |
| `max_entries_per_batch` | usize | `256` | Entries per flush batch |

### `worker`

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `concurrency` | u32 | `4` | Worker goroutine concurrency |
| `heartbeat_ms` | u64 | `5000` | Worker heartbeat interval |
| `provider` | string | `"openai"` | Default LLM provider name |
| `api_key_ref` | string? | `null` | Secret ref: `"env:OPENAI_API_KEY"` or `"file:api_key"` |

### `telemetry`

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `metrics_enabled` | bool | `true` | Emit Prometheus metrics |
| `tracing_enabled` | bool | `false` | Emit OTLP traces |
| `scrape_interval_ms` | u64 | `15000` | Metrics scrape interval |

## Hot-reloadable fields

The following fields can be changed at runtime without restarting workers:

- `core.log_level`
- `telemetry.metrics_enabled`
- `telemetry.tracing_enabled`
- `telemetry.scrape_interval_ms`

All other fields require a full process restart.

## Environment overlay

Use the env overlay to inject environment-specific settings. Field semantics are identical; pass a partial `AncoraCfg` with only the fields you wish to override.

## Tenant overlays

Register per-tenant overrides with `TenantOverlayRegistry::register`. Supported per-tenant fields:

- `worker.concurrency`
- `worker.provider`
- `worker.api_key_ref`
- `core.max_concurrent_runs`

## Config dump

Call `redacted_dump(cfg)` to obtain a JSON representation safe for logging. All fields whose names contain `ref`, `key`, or `secret` are replaced with `[REDACTED]`.

## Schema validation

`validate(cfg)` returns `Err(ConfigError::Validation { field, reason })` for any rule violation. Always validate before writing config to disk or applying a live reload.
