# Telemetry Privacy Guide

## Overview

Ancora telemetry is redaction-first. Sensitive data never leaves the process
boundary unless the operator has explicitly opted in. This guide explains the
privacy model and how to reason about what gets exported.

## Data Classification

Every telemetry attribute is classified at one of four levels:

| Class | Meaning | Default action |
|-------|---------|----------------|
| Public | Freely shareable | Export as-is |
| Internal | Internal metrics | Export as-is |
| Sensitive | PII or confidential | Redact to `[REDACTED]` |
| Critical | Credentials, keys | Redact to `[REDACTED]` |

The classification is determined by:
1. The attribute name (heuristic classifier in `classification.rs`).
2. Explicit labels set by the emitting code.
3. The allowlist in `allowlist.rs`.

## What Is Never Exported by Default

- Raw prompt text
- Raw completion text
- User email addresses or phone numbers
- IP addresses
- API keys or secrets
- Social Security Numbers

## PII Scrubbing

Even for attributes that pass the classification check, the `pii_scrub` module
runs a final pass over string values and replaces detected PII with `[PII]`.
Patterns detected include email addresses, IPv4 addresses, US phone numbers
(10-digit blocks), and SSNs in NNN-NN-NNNN format.

## Opting In

To export prompt or completion text (for debugging or eval workflows), set the
appropriate `OptInFeature` flag in your `OptInRegistry`. See
[configuring-redaction.md](configuring-redaction.md) for details.
