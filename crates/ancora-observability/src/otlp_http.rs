use std::sync::{Arc, Mutex};

use crate::exporter::SpanEmitter;
use crate::span::Span;

/// Error returned when an OTLP HTTP export fails.
#[derive(Debug)]
pub struct OtlpHttpError(pub String);

impl std::fmt::Display for OtlpHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "otlp http error: {}", self.0)
    }
}

impl std::error::Error for OtlpHttpError {}

/// Buffers spans and exports them to an OTLP HTTP endpoint.
pub struct OtlpHttpExporter {
    endpoint: String,
    buffer: Arc<Mutex<Vec<Span>>>,
}

impl OtlpHttpExporter {
    /// Create a new exporter pointing at the given OTLP HTTP endpoint.
    /// Typical default: `http://localhost:4318/v1/traces`.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Flush buffered spans to the OTLP HTTP endpoint.
    pub fn export(&self) -> Result<(), OtlpHttpError> {
        let spans: Vec<Span> = {
            let mut buf = self.buffer.lock().unwrap();
            std::mem::take(&mut *buf)
        };
        if spans.is_empty() {
            return Ok(());
        }
        let body = crate::otlp::spans_to_otlp(&spans);
        let json = serde_json::to_string(&body).map_err(|e| OtlpHttpError(e.to_string()))?;
        ureq::post(&self.endpoint)
            .set("Content-Type", "application/json")
            .send_string(&json)
            .map_err(|e| OtlpHttpError(e.to_string()))?;
        Ok(())
    }

    /// Returns the configured endpoint URL.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Buffered span count (for testing).
    pub fn buffered(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
}

impl SpanEmitter for OtlpHttpExporter {
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
    fn http_exporter_buffers_spans_before_export() {
        let exp = OtlpHttpExporter::new("http://localhost:1");
        exp.emit(Span::new("a"));
        exp.emit(Span::new("b"));
        assert_eq!(exp.buffered(), 2);
    }

    #[test]
    fn http_exporter_clears_buffer_on_failed_export() {
        let exp = OtlpHttpExporter::new("http://localhost:1");
        exp.emit(Span::new("x"));
        let _ = exp.export();
        assert_eq!(exp.buffered(), 0);
    }

    #[test]
    fn http_exporter_export_empty_is_noop() {
        let exp = OtlpHttpExporter::new("http://localhost:1");
        assert!(exp.export().is_ok());
    }

    #[test]
    fn http_exporter_endpoint_is_accessible() {
        let exp = OtlpHttpExporter::new("http://localhost:4318/v1/traces");
        assert_eq!(exp.endpoint(), "http://localhost:4318/v1/traces");
    }
}
