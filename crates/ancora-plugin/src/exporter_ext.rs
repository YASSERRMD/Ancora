/// Exporter extension point - forward telemetry spans and metrics to an external sink.
use std::collections::HashMap;

/// A telemetry span representing a unit of work.
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub attributes: HashMap<String, String>,
    pub status: SpanStatus,
}

/// The completion status of a span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanStatus {
    Ok,
    Error(String),
}

/// A named metric data point.
#[derive(Debug, Clone)]
pub struct MetricPoint {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp_ns: u64,
}

/// Error from an exporter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    ConnectionFailed(String),
    Serialization(String),
    Quota,
    Unknown(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::ConnectionFailed(s) => write!(f, "connection failed: {s}"),
            ExportError::Serialization(s) => write!(f, "serialization error: {s}"),
            ExportError::Quota => write!(f, "export quota exceeded"),
            ExportError::Unknown(s) => write!(f, "unknown export error: {s}"),
        }
    }
}

impl std::error::Error for ExportError {}

/// Trait that exporter plugins must implement.
pub trait ExporterPlugin: Send + Sync {
    fn exporter_id(&self) -> &str;

    /// Export a batch of spans.
    fn export_spans(&self, spans: &[Span]) -> Result<(), ExportError>;

    /// Export a batch of metric points.
    fn export_metrics(&self, metrics: &[MetricPoint]) -> Result<(), ExportError>;

    /// Flush any buffered data. Called during graceful shutdown.
    fn flush(&self) -> Result<(), ExportError> {
        Ok(())
    }
}

/// A no-op exporter that discards all telemetry; useful in tests.
pub struct NoopExporter {
    id: String,
    /// Count of exported spans (useful for assertions).
    pub span_count: std::sync::atomic::AtomicUsize,
    pub metric_count: std::sync::atomic::AtomicUsize,
}

impl NoopExporter {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            span_count: std::sync::atomic::AtomicUsize::new(0),
            metric_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl ExporterPlugin for NoopExporter {
    fn exporter_id(&self) -> &str {
        &self.id
    }

    fn export_spans(&self, spans: &[Span]) -> Result<(), ExportError> {
        self.span_count
            .fetch_add(spans.len(), std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn export_metrics(&self, metrics: &[MetricPoint]) -> Result<(), ExportError> {
        self.metric_count
            .fetch_add(metrics.len(), std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}
