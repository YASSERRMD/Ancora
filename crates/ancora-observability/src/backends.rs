//! Backend configuration helpers for common OTLP-compatible collectors.
//!
//! # Phoenix (Arize)
//! OTLP HTTP: `http://localhost:6006/v1/traces`
//! OTLP gRPC: `http://localhost:4317`
//!
//! # Langfuse
//! Use the OpenTelemetry-compatible ingest endpoint provided in your
//! project settings under "Tracing". Set the `Authorization` header to
//! `Basic <base64(pk:sk)>`.
//!
//! # Grafana Tempo
//! OTLP HTTP: `http://localhost:4318/v1/traces`
//! OTLP gRPC: `http://localhost:4317`
//!
//! # Jaeger
//! OTLP HTTP: `http://localhost:4318/v1/traces`
//! OTLP gRPC: `http://localhost:4317`

/// OTLP HTTP default endpoint for Grafana Tempo and Jaeger.
pub const OTLP_HTTP_DEFAULT: &str = "http://localhost:4318/v1/traces";

/// OTLP gRPC default endpoint for Grafana Tempo and Jaeger.
pub const OTLP_GRPC_DEFAULT: &str = "http://localhost:4317";

/// Phoenix OTLP HTTP endpoint.
pub const PHOENIX_HTTP_DEFAULT: &str = "http://localhost:6006/v1/traces";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otlp_http_default_endpoint_is_port_4318() {
        assert!(OTLP_HTTP_DEFAULT.contains(":4318"));
    }

    #[test]
    fn otlp_grpc_default_endpoint_is_port_4317() {
        assert!(OTLP_GRPC_DEFAULT.contains(":4317"));
    }

    #[test]
    fn phoenix_http_default_endpoint_is_port_6006() {
        assert!(PHOENIX_HTTP_DEFAULT.contains(":6006"));
    }
}
