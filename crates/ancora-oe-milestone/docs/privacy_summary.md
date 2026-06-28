# Privacy Posture Summary

This document summarises the privacy posture of the Ancora observability
and eval data pipeline as of the 0.6.0 milestone.

## Data Classification

| Signal | Contains PII | Scrubbing | Retention |
| --- | --- | --- | --- |
| Traces | Possible (span attributes) | Enabled (Rust, Python, TS) | 14 days |
| Metrics | No | N/A | 30 days |
| Logs | Possible (message body) | Configurable | 14 days |
| Eval inputs | Yes (prompt content) | Enabled | 7 days |
| Eval outputs | Possible | Enabled | 7 days |

## Encryption

- Data at rest: AES-256-GCM (all regions)
- Data in transit: TLS 1.3 enforced

## Access Controls

- Tenant isolation via row-level security
- RBAC on all signal query APIs
- Audit log of all data access events

## Residency Options

- us-east (default)
- eu-west (GDPR-compliant)
- ap-southeast
- self-hosted (customer-managed)

## Compliance Status

| Standard | Status |
| --- | --- |
| SOC 2 Type II | Certified |
| GDPR | Compliant (eu-west region) |
| HIPAA | In progress |
| ISO 27001 | Planned |

Last updated: 2026-06-29
