/// ancora-contrib: exporter template
///
/// Copy this module as the starting point for a new telemetry/trace exporter.
/// Rename `MyExporter` and implement `export`.

/// A single span representing a unit of work in an agent run.
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub attributes: Vec<(String, String)>,
    pub status: SpanStatus,
}

impl Span {
    pub fn new(
        trace_id: impl Into<String>,
        span_id: impl Into<String>,
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
    ) -> Self {
        Self {
            trace_id: trace_id.into(),
            span_id: span_id.into(),
            parent_id: None,
            name: name.into(),
            start_ns,
            end_ns,
            attributes: Vec::new(),
            status: SpanStatus::Ok,
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    pub fn duration_ns(&self) -> u64 {
        self.end_ns.saturating_sub(self.start_ns)
    }
}

/// Outcome status for a span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanStatus {
    Ok,
    Error(String),
}

/// A batch of spans ready for export.
#[derive(Debug, Clone, Default)]
pub struct SpanBatch {
    pub spans: Vec<Span>,
}

impl SpanBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }
}

/// Errors an exporter may return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExporterError {
    ConnectionFailed(String),
    SerializationFailed(String),
    Backpressure,
}

impl std::fmt::Display for ExporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExporterError::ConnectionFailed(s) => write!(f, "connection failed: {s}"),
            ExporterError::SerializationFailed(s) => write!(f, "serialization failed: {s}"),
            ExporterError::Backpressure => write!(f, "exporter backpressure; try again later"),
        }
    }
}

impl std::error::Error for ExporterError {}

/// Trait all exporter plugins must implement.
pub trait ExporterPlugin: Send + Sync {
    /// Stable identifier (e.g. "otlp-grpc", "datadog", "file-json").
    fn exporter_id(&self) -> &str;

    /// Export a batch of spans to the target backend.
    fn export(&self, batch: SpanBatch) -> Result<(), ExporterError>;

    /// Flush any buffered data and release resources.
    fn shutdown(&self) -> Result<(), ExporterError>;
}

// ---------------------------------------------------------------------------
// Template implementation
// ---------------------------------------------------------------------------

/// Template exporter: stores spans in memory for test inspection.
pub struct MyExporter {
    pub name: String,
    exported: std::sync::Mutex<Vec<Span>>,
}

impl MyExporter {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), exported: std::sync::Mutex::new(Vec::new()) }
    }

    /// Return a snapshot of all exported spans (for tests).
    pub fn exported_spans(&self) -> Vec<Span> {
        self.exported.lock().unwrap().clone()
    }

    pub fn exported_count(&self) -> usize {
        self.exported.lock().unwrap().len()
    }
}

impl ExporterPlugin for MyExporter {
    fn exporter_id(&self) -> &str {
        // TODO: replace with your exporter's identifier.
        "my-exporter"
    }

    fn export(&self, batch: SpanBatch) -> Result<(), ExporterError> {
        // TODO: replace with real export logic (HTTP, gRPC, file, etc.).
        let mut guard = self.exported.lock().unwrap();
        for span in batch.spans {
            guard.push(span);
        }
        Ok(())
    }

    fn shutdown(&self) -> Result<(), ExporterError> {
        // TODO: flush buffers and close connections.
        Ok(())
    }
}
