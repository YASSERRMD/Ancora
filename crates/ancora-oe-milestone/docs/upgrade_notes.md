# Upgrade Notes: 0.5.x to 0.6.0

## [BREAKING] Renamed trace exporter config key

The configuration key `otel_endpoint` has been renamed to `exporter_endpoint`
to align with the upstream OpenTelemetry naming conventions.

**Migration steps:**
1. Open your `ancora.toml` or environment config.
2. Replace `otel_endpoint` with `exporter_endpoint`.
3. Restart the agent process.

---

## [non-breaking] Histogram bucket defaults updated

Default histogram bucket boundaries now follow the OpenTelemetry recommended
set. Existing custom bucket configs are unaffected.

---

## [non-breaking] Eval gating threshold syntax

Eval gate thresholds now accept a `p` prefix for percentile notation:
`p50`, `p90`, `p99`. The previous integer-only form still works.

---

## [non-breaking] Metrics retention extended

Default cloud-hosted metrics retention has been extended from 15 days to
30 days at no additional cost.

Last updated: 2026-06-29
