# Observability and Eval Parity Matrix

This document tracks parity status for each observability and eval feature across all six language SDKs.

## Languages

| Code | SDK |
|------|-----|
| RS   | Rust |
| PY   | Python |
| TS   | TypeScript |
| GO   | Go |
| JV   | Java |
| CS   | C# |

## Feature Parity Matrix

| Feature            | RS | PY | TS | GO | JV | CS | Status |
|--------------------|----|----|----|----|----|----|--------|
| Trace emission     | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Span attributes    | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Cost attributes    | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Eval run           | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Graders            | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Regression gates   | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Drift detection    | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Feedback capture   | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| PII redaction      | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| OTLP export        | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |
| Polyglot stitching | Y  | Y  | Y  | Y  | Y  | Y  | PASS   |

## Score Parity

All six SDKs must produce mean_score within 0.01 of each other on the shared eval dataset `ancora-oepar-v1`.

## Trace Parity

Traces are compared on: span count, span names, required attributes. Parent link integrity is enforced.
