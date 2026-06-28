# Security, Policy, and Sovereignty Test Plan

This document describes the security and policy tests introduced in Phase 155. All tests run offline -- no network calls, no real keys, no live stores.

## Security tests

| File | Coverage |
|---|---|
| `sec_prompt_injection.rs` | Detect injection patterns, case-insensitive, sanitise control chars |
| `sec_data_exfiltration.rs` | PII guard blocks SSN, api_key, password, token, secret in outbound payloads |
| `sec_tool_allowlist.rs` | Only allowlisted tools may be invoked, exact match, empty list rejects all |
| `sec_output_filter.rs` | Redact sensitive fields before returning to caller, multiple fields at once |
| `sec_key_rotation.rs` | New key encrypts, old key still decrypts old ciphertext, unknown key id errors |
| `sec_audit_log.rs` | Every operation logged, seq monotonic, denied ops captured, retrieve by seq |
| `sec_rbac.rs` | Viewer/operator/editor/admin permissions, unknown role has no permissions |
| `sec_no_live_keys.rs` | No sk-ant-, sk-, AIza, ghp_, xoxb- prefixes in any fixture blob |
| `sec_input_size_limit.rs` | 1 MiB input limit, 64 KiB tool result limit, error includes sizes |
| `sec_tls_config.rs` | Minimum TLS 1.2, TLS 1.0/1.1 rejected, strong cipher allowlist |

## Policy tests

| File | Coverage |
|---|---|
| `policy_data_residency.rs` | Allowed regions pass, disallowed regions fail, empty list rejects all |
| `policy_model_allowlist.rs` | Approved model IDs pass, unapproved rejected, partial match rejected |
| `policy_cost_ceiling.rs` | Accumulate cost, reject when ceiling exceeded, zero ceiling rejects any |
| `policy_retention.rs` | Records older than TTL eligible, boundary not eligible, apply bulk |
| `policy_sovereignty.rs` | Local-only mode blocks remote calls, error includes host name |
| `policy_consent.rs` | Destructive action blocked without consent, granted allows, denied blocks |
| `policy_gdpr_erasure.rs` | Erase user purges all entries, returns count, double erasure safe |
| `policy_offline_mode.rs` | All providers must have local_only=true, violations named in error |

## Running the suite

```bash
cargo test -p ancora-core \
  --test sec_prompt_injection \
  --test sec_data_exfiltration \
  --test sec_tool_allowlist \
  --test sec_output_filter \
  --test sec_key_rotation \
  --test sec_audit_log \
  --test sec_rbac \
  --test sec_no_live_keys \
  --test sec_input_size_limit \
  --test sec_tls_config \
  --test policy_data_residency \
  --test policy_model_allowlist \
  --test policy_cost_ceiling \
  --test policy_retention \
  --test policy_sovereignty \
  --test policy_consent \
  --test policy_gdpr_erasure \
  --test policy_offline_mode
```

All tests run offline by default.
