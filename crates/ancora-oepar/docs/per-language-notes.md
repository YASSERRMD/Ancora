# Per-Language Observability and Eval Notes

## Rust

- Traces emitted via `ancora-trace` crate using the OpenTelemetry SDK.
- Cost attributes computed by `ancora-costan`.
- Eval runs use `ancora-evalrun`.
- Graders implemented as Rust traits in `ancora-evals`.

## Python

- Traces emitted using `opentelemetry-sdk` with the Ancora Python wrapper.
- Cost attributes follow the `gen_ai.*` semantic conventions.
- Eval dataset loaded from the shared `ancora-oepar-v1` JSON fixture.

## TypeScript

- Traces emitted via `@opentelemetry/sdk-node`.
- Eval runs execute in Node.js with the shared fixture loaded at startup.
- Feedback events serialized as JSON and flushed on process exit.

## Go

- Traces emitted using `go.opentelemetry.io/otel`.
- Grader implementations mirror the Rust F1 and exact-match logic.
- Drift detection uses a sliding window with the same capacity (100 samples).

## Java

- Traces emitted via `opentelemetry-java`.
- Cost attributes computed using BigDecimal to avoid floating-point drift.
- Eval summaries serialized as protobuf for CI comparison.

## C# (dotnet)

- Traces emitted via `OpenTelemetry.Extensions.Hosting`.
- Redaction rules implemented as `Regex.Replace` with the same patterns.
- Regression gates use the same baseline thresholds as Rust.

## Shared Conventions

- All SDKs emit `gen_ai.system = "ancora"` on every span.
- Token counts and costs must match within 1e-9 USD across languages.
- Eval case IDs are stable strings prefixed `case-` (e.g. `case-001`).
