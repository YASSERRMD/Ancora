# Quota Reference

## ResourceQuota fields

| Field | Type | Description |
|---|---|---|
| `max_agents` | `u64` | Maximum concurrent agents the tenant may spawn |
| `max_tasks` | `u64` | Maximum concurrent tasks |
| `max_memory_mb` | `u64` | Maximum total memory allocation in megabytes |
| `max_cpu_millicores` | `u64` | Maximum CPU allocation in millicores (1000 = 1 core) |
| `max_secrets` | `u64` | Maximum secrets stored in the tenant's vault |
| `max_log_entries` | `u64` | Maximum in-memory audit log entries |

## Built-in presets

| Preset | max_agents | max_tasks | max_memory_mb | max_secrets |
|---|---|---|---|---|
| `standard()` | 10 | 100 | 4096 | 50 |
| `restricted()` | 2 | 20 | 512 | 10 |
| `unlimited()` | u64::MAX | u64::MAX | u64::MAX | u64::MAX |

## AdmissionController methods

| Method | Checks |
|---|---|
| `check_agents(quota, usage, delta)` | `usage.agents + delta <= quota.max_agents` |
| `check_tasks(quota, usage, delta)` | `usage.tasks + delta <= quota.max_tasks` |
| `check_memory(quota, usage, delta_mb)` | `usage.memory_mb + delta_mb <= quota.max_memory_mb` |
| `check_secrets(quota, usage, delta)` | `usage.secrets + delta <= quota.max_secrets` |
| `check_log_entries(quota, usage, delta)` | `usage.log_entries + delta <= quota.max_log_entries` |

All methods return `AdmissionDecision::Allow` or `AdmissionDecision::Deny(String)`.

## Updating quotas at runtime

Use `QuotaUpdate` to change individual quota fields without constructing a new `ResourceQuota`:

```rust
use ancora_tenant::QuotaUpdate;

QuotaUpdate::new().agents(50).memory_mb(8192).apply(&mut quota);
```
