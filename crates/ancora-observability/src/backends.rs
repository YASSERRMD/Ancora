/// Backend configuration helpers for common OTLP-compatible collectors.
///
/// # Phoenix (Arize)
/// OTLP HTTP: `http://localhost:6006/v1/traces`
/// OTLP gRPC: `http://localhost:4317`
///
/// # Langfuse
/// Use the OpenTelemetry-compatible ingest endpoint provided in your
/// project settings under "Tracing". Set the `Authorization` header to
/// `Basic <base64(pk:sk)>`.
///
/// # Grafana Tempo
/// OTLP HTTP: `http://localhost:4318/v1/traces`
/// OTLP gRPC: `http://localhost:4317`
///
/// # Jaeger
/// OTLP HTTP: `http://localhost:4318/v1/traces`
/// OTLP gRPC: `http://localhost:4317`

/// OTLP HTTP default endpoint for Grafana Tempo and Jaeger.
pub const OTLP_HTTP_DEFAULT: &str = "http://localhost:4318/v1/traces";

/// OTLP gRPC default endpoint for Grafana Tempo and Jaeger.
pub const OTLP_GRPC_DEFAULT: &str = "http://localhost:4317";

/// Phoenix OTLP HTTP endpoint.
pub const PHOENIX_HTTP_DEFAULT: &str = "http://localhost:6006/v1/traces";
