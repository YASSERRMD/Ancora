//! Trace model definitions for agent spans and propagation.

/// Identifies a single distributed trace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraceId(pub String);

/// Identifies a single span within a trace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanId(pub String);

/// The kind of span being recorded.
#[derive(Debug, Clone, PartialEq)]
pub enum SpanKind {
    /// Top-level agent invocation.
    Agent,
    /// Tool call initiated by the agent.
    Tool,
    /// LLM inference request.
    Llm,
    /// Internal logic span.
    Internal,
}

/// A single span in a distributed trace.
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub name: String,
    pub kind: SpanKind,
    pub start_ns: u64,
    pub end_ns: Option<u64>,
}

impl Span {
    /// Create a new root span.
    pub fn root(trace_id: TraceId, span_id: SpanId, name: impl Into<String>) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            name: name.into(),
            kind: SpanKind::Agent,
            start_ns: 0,
            end_ns: None,
        }
    }

    /// Returns the duration in nanoseconds if the span has finished.
    pub fn duration_ns(&self) -> Option<u64> {
        self.end_ns.map(|e| e.saturating_sub(self.start_ns))
    }

    /// Mark the span as finished at the given timestamp.
    pub fn finish(&mut self, end_ns: u64) {
        self.end_ns = Some(end_ns);
    }
}

/// A complete trace composed of multiple spans.
#[derive(Debug, Default)]
pub struct Trace {
    pub spans: Vec<Span>,
}

impl Trace {
    /// Add a span to the trace.
    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Return spans matching the given kind.
    pub fn spans_of_kind(&self, kind: &SpanKind) -> Vec<&Span> {
        self.spans.iter().filter(|s| &s.kind == kind).collect()
    }
}
