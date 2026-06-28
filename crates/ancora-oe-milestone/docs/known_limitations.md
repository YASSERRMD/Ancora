# Known Limitations

## LIM-001: High-cardinality label explosion (severity: high)

Metrics with more than 10,000 unique label value combinations may exceed
storage backend limits and cause ingestion failures.

**Workaround:** Use the label allow-list configuration to cap cardinality.
**Tracking:** ancora/issues/891

---

## LIM-002: Go SDK log correlation (severity: medium)

Log-to-trace correlation is currently in Beta for the Go SDK. The
`trace_id` injection into structured log records may be absent in some
goroutine-pool configurations.

**Workaround:** Manually attach `trace_id` from `context.Context`.
**Tracking:** ancora/issues/904

---

## LIM-003: Self-hosted exporter auth (severity: medium)

The self-hosted OTLP exporter does not yet support mTLS client certificates.
Only bearer-token auth and unauthenticated modes are supported.

**Workaround:** Terminate TLS at a sidecar proxy.
**Tracking:** ancora/issues/912

---

## LIM-004: Privacy label scrubbing - Go (severity: low)

PII label scrubbing is not yet implemented for the Go SDK (Planned).

**Workaround:** Apply scrubbing at the collector level.
**Tracking:** ancora/issues/921
