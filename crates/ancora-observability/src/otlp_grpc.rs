use std::sync::{Arc, Mutex};

use crate::exporter::SpanEmitter;
use crate::span::Span;

/// Error returned when a gRPC export fails.
#[derive(Debug)]
pub struct OtlpGrpcError(pub String);

impl std::fmt::Display for OtlpGrpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "otlp grpc error: {}", self.0)
    }
}

impl std::error::Error for OtlpGrpcError {}

/// Buffers spans and exports them to an OTLP gRPC endpoint via HTTP/2 framing.
///
/// Full gRPC over HTTP/2 with Protobuf encoding requires a runtime such as
/// tonic. This stub buffers spans and delegates to an OTLP HTTP/JSON proxy
/// when `export` is called, making it compatible with gRPC-compatible
/// collectors that expose both protocols (Jaeger, Phoenix, Grafana Tempo).
pub struct OtlpGrpcExporter {
    endpoint: String,
    buffer: Arc<Mutex<Vec<Span>>>,
}

impl OtlpGrpcExporter {
    /// Create a new exporter pointing at the given gRPC endpoint.
    /// Typical default: `http://localhost:4317`.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns the configured endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Flush buffered spans via the OTLP HTTP/JSON protocol on the same host.
    pub fn export(&self) -> Result<(), OtlpGrpcError> {
        let spans: Vec<Span> = {
            let mut buf = self.buffer.lock().unwrap();
            std::mem::take(&mut *buf)
        };
        if spans.is_empty() {
            return Ok(());
        }
        let body = crate::otlp::spans_to_otlp(&spans);
        let json = serde_json::to_string(&body).map_err(|e| OtlpGrpcError(e.to_string()))?;
        let http_endpoint = self.endpoint.replace(":4317", ":4318") + "/v1/traces";
        ureq::post(&http_endpoint)
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| OtlpGrpcError(e.to_string()))?;
        Ok(())
    }

    /// Buffered span count (for testing).
    pub fn buffered(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
}

impl SpanEmitter for OtlpGrpcExporter {
    fn emit(&self, span: Span) {
        self.buffer.lock().unwrap().push(span);
    }

    fn flush(&self) {
        let _ = self.export();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exporter::SpanEmitter;
    use crate::span::Span;

    #[test]
    fn grpc_exporter_stores_endpoint() {
        let exp = OtlpGrpcExporter::new("http://localhost:4317");
        assert_eq!(exp.endpoint(), "http://localhost:4317");
    }

    #[test]
    fn grpc_exporter_buffers_spans() {
        let exp = OtlpGrpcExporter::new("http://localhost:4317");
        exp.emit(Span::new("grpc-span"));
        assert_eq!(exp.buffered(), 1);
    }

    #[test]
    fn grpc_exporter_export_empty_is_noop() {
        let exp = OtlpGrpcExporter::new("http://localhost:4317");
        assert!(exp.export().is_ok());
    }

    #[test]
    fn grpc_exporter_clears_buffer_after_export() {
        let exp = OtlpGrpcExporter::new("http://localhost:1");
        exp.emit(Span::new("x"));
        let _ = exp.export();
        assert_eq!(exp.buffered(), 0);
    }
}
