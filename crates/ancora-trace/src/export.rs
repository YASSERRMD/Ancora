/// OTLP-compatible trace export.
///
/// Serialises spans into a simple wire-compatible format and dispatches
/// them to a configurable collector endpoint.  The actual HTTP transport
/// is stubbed so that tests can run without network access; real export
/// would replace the stub with a proper OTLP/HTTP or gRPC transport.
use crate::span::{AttributeValue, Span, SpanStatus};
use crate::trace::Trace;

/// Minimal OTLP-like resource descriptor.
#[derive(Debug, Clone)]
pub struct Resource {
    pub service_name: String,
    pub service_version: String,
    pub attributes: Vec<(String, String)>,
}

impl Resource {
    pub fn new(service_name: &str, service_version: &str) -> Self {
        Resource {
            service_name: service_name.to_owned(),
            service_version: service_version.to_owned(),
            attributes: Vec::new(),
        }
    }

    pub fn with_attr(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.to_owned(), value.to_owned()));
        self
    }
}

/// A span serialised to a flat, OTLP-compatible record.
#[derive(Debug, Clone)]
pub struct ExportedSpan {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_unix_ns: u64,
    pub end_unix_ns: u64,
    pub status_code: u32,
    pub status_message: String,
    pub attributes: Vec<(String, String)>,
}

/// Convert a `Span` to an `ExportedSpan`.
pub fn span_to_exported(span: &Span) -> ExportedSpan {
    let (status_code, status_message) = match &span.status {
        SpanStatus::Ok => (1, String::new()),
        SpanStatus::Unset => (0, String::new()),
        SpanStatus::Error { code, message } => (*code, message.clone()),
    };

    let attributes: Vec<(String, String)> = span
        .attributes
        .iter()
        .map(|(k, v)| {
            let vs = match v {
                AttributeValue::String(s) => s.clone(),
                AttributeValue::Int(n) => n.to_string(),
                AttributeValue::Float(f) => format!("{:.6}", f),
                AttributeValue::Bool(b) => b.to_string(),
            };
            (k.clone(), vs)
        })
        .collect();

    ExportedSpan {
        trace_id: span.trace_id.0.clone(),
        span_id: span.span_id.0.clone(),
        parent_span_id: span.parent_id.as_ref().map(|p| p.0.clone()),
        name: span.name.clone(),
        start_unix_ns: span.start_ns,
        end_unix_ns: span.end_ns.unwrap_or(span.start_ns),
        status_code,
        status_message,
        attributes,
    }
}

/// An export batch ready for transmission.
#[derive(Debug, Clone)]
pub struct ExportBatch {
    pub resource: Resource,
    pub spans: Vec<ExportedSpan>,
}

impl ExportBatch {
    pub fn from_trace(trace: &Trace, resource: Resource) -> Self {
        let spans: Vec<ExportedSpan> = trace
            .all_spans()
            .into_iter()
            .map(span_to_exported)
            .collect();
        ExportBatch { resource, spans }
    }
}

/// Outcome of an export attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportResult {
    Success { span_count: usize },
    Failure { reason: String },
}

/// Trait for OTLP span exporters.
pub trait SpanExporter: Send + Sync {
    fn export(&self, batch: &ExportBatch) -> ExportResult;
}

/// A no-op exporter that accepts all spans and discards them.
pub struct NoopExporter;

impl SpanExporter for NoopExporter {
    fn export(&self, batch: &ExportBatch) -> ExportResult {
        ExportResult::Success {
            span_count: batch.spans.len(),
        }
    }
}

/// A mock exporter that records all exported spans in memory.
///
/// Useful for unit tests that verify export output without network access.
pub struct MockCollector {
    inner: std::sync::Mutex<Vec<ExportedSpan>>,
}

impl MockCollector {
    pub fn new() -> Self {
        MockCollector {
            inner: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Return a snapshot of all received spans.
    pub fn collected(&self) -> Vec<ExportedSpan> {
        self.inner.lock().unwrap().clone()
    }

    /// Clear the collected spans.
    pub fn clear(&self) {
        self.inner.lock().unwrap().clear();
    }
}

impl Default for MockCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SpanExporter for MockCollector {
    fn export(&self, batch: &ExportBatch) -> ExportResult {
        let mut guard = self.inner.lock().unwrap();
        guard.extend(batch.spans.iter().cloned());
        ExportResult::Success {
            span_count: batch.spans.len(),
        }
    }
}

/// Configuration for the OTLP exporter.
#[derive(Debug, Clone)]
pub struct OtlpConfig {
    pub endpoint: String,
    pub headers: Vec<(String, String)>,
    pub timeout_ms: u64,
}

impl OtlpConfig {
    pub fn new(endpoint: &str) -> Self {
        OtlpConfig {
            endpoint: endpoint.to_owned(),
            headers: Vec::new(),
            timeout_ms: 5_000,
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_owned(), value.to_owned()));
        self
    }
}

/// Stub OTLP HTTP exporter.  In production this would open a TCP
/// connection; here it delegates to an inner exporter for testability.
pub struct OtlpExporter<E: SpanExporter> {
    pub config: OtlpConfig,
    inner: E,
}

impl<E: SpanExporter> OtlpExporter<E> {
    pub fn new(config: OtlpConfig, inner: E) -> Self {
        OtlpExporter { config, inner }
    }
}

impl<E: SpanExporter> SpanExporter for OtlpExporter<E> {
    fn export(&self, batch: &ExportBatch) -> ExportResult {
        self.inner.export(batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{Span, SpanStatus};
    use crate::trace::Trace;

    #[test]
    fn noop_exporter_accepts_empty_batch() {
        let resource = Resource::new("ancora", "0.1.0");
        let root = Span::root("root", 0);
        let tid = root.trace_id.clone();
        let trace = Trace::new(tid, root);
        let batch = ExportBatch::from_trace(&trace, resource);
        let result = NoopExporter.export(&batch);
        assert_eq!(result, ExportResult::Success { span_count: 1 });
    }

    #[test]
    fn mock_collector_records_spans() {
        let resource = Resource::new("ancora", "0.1.0");
        let mut root = Span::root("root", 0);
        root.finish(1000, SpanStatus::Ok);
        let tid = root.trace_id.clone();
        let trace = Trace::new(tid, root);
        let batch = ExportBatch::from_trace(&trace, resource);
        let collector = MockCollector::new();
        let result = collector.export(&batch);
        assert_eq!(result, ExportResult::Success { span_count: 1 });
        assert_eq!(collector.collected().len(), 1);
    }
}
