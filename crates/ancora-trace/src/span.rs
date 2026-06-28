/// Span types for the Ancora trace model.
///
/// A span represents a single unit of work within a trace, such as a tool
/// invocation, an LLM call, or an agent-to-agent (a2a) handoff.

use std::collections::HashMap;

/// Opaque 128-bit trace identifier (stored as hex string for simplicity).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraceId(pub String);

/// Opaque 64-bit span identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanId(pub String);

/// Status of a completed span.
#[derive(Debug, Clone, PartialEq)]
pub enum SpanStatus {
    Ok,
    Error { code: u32, message: String },
    Unset,
}

/// The kind of work this span represents.
#[derive(Debug, Clone, PartialEq)]
pub enum SpanKind {
    /// Internal agent computation.
    Internal,
    /// An outbound call to a tool or external service.
    Client,
    /// An inbound request handled by this agent.
    Server,
    /// An agent-to-agent invocation.
    Producer,
    /// Receiving side of an a2a call.
    Consumer,
}

/// A single span within a distributed trace.
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_id: Option<SpanId>,
    pub name: String,
    pub kind: SpanKind,
    /// Nanoseconds since UNIX epoch.
    pub start_ns: u64,
    /// None while the span is still open.
    pub end_ns: Option<u64>,
    pub status: SpanStatus,
    /// Arbitrary key-value attributes (genai conventions, cost, etc.).
    pub attributes: HashMap<String, AttributeValue>,
    /// Structured events within the span lifetime.
    pub events: Vec<SpanEvent>,
    /// Number of retry attempts recorded on this span.
    pub retry_count: u32,
}

/// A typed attribute value.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl AttributeValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            AttributeValue::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            AttributeValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

/// A timestamped event within a span.
#[derive(Debug, Clone)]
pub struct SpanEvent {
    pub name: String,
    pub timestamp_ns: u64,
    pub attributes: HashMap<String, AttributeValue>,
}

impl Span {
    /// Create a root span (no parent).
    pub fn root(name: &str, start_ns: u64) -> Self {
        let trace_id = TraceId(format!("{:032x}", start_ns ^ 0xdeadbeef_cafebabe));
        let span_id = SpanId(format!("{:016x}", start_ns ^ 0xfeed_face));
        Span {
            trace_id,
            span_id,
            parent_id: None,
            name: name.to_owned(),
            kind: SpanKind::Internal,
            start_ns,
            end_ns: None,
            status: SpanStatus::Unset,
            attributes: HashMap::new(),
            events: Vec::new(),
            retry_count: 0,
        }
    }

    /// Create a child span linked to a parent.
    pub fn child(name: &str, parent_id: SpanId, trace_id: TraceId, start_ns: u64) -> Self {
        let span_id = SpanId(format!("{:016x}", start_ns ^ 0xabcd_1234));
        Span {
            trace_id,
            span_id,
            parent_id: Some(parent_id),
            name: name.to_owned(),
            kind: SpanKind::Internal,
            start_ns,
            end_ns: None,
            status: SpanStatus::Unset,
            attributes: HashMap::new(),
            events: Vec::new(),
            retry_count: 0,
        }
    }

    /// Set a string attribute.
    pub fn set_attr_str(&mut self, key: &str, value: impl Into<String>) {
        self.attributes
            .insert(key.to_owned(), AttributeValue::String(value.into()));
    }

    /// Set an integer attribute.
    pub fn set_attr_int(&mut self, key: &str, value: i64) {
        self.attributes
            .insert(key.to_owned(), AttributeValue::Int(value));
    }

    /// Set a float attribute.
    pub fn set_attr_float(&mut self, key: &str, value: f64) {
        self.attributes
            .insert(key.to_owned(), AttributeValue::Float(value));
    }

    /// Mark the span as finished.
    pub fn finish(&mut self, end_ns: u64, status: SpanStatus) {
        self.end_ns = Some(end_ns);
        self.status = status;
    }

    /// Duration in nanoseconds, if finished.
    pub fn duration_ns(&self) -> Option<u64> {
        self.end_ns.map(|e| e.saturating_sub(self.start_ns))
    }

    /// Add a structured event.
    pub fn add_event(&mut self, name: &str, timestamp_ns: u64) {
        self.events.push(SpanEvent {
            name: name.to_owned(),
            timestamp_ns,
            attributes: HashMap::new(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_span_has_no_parent() {
        let s = Span::root("test", 0);
        assert!(s.parent_id.is_none());
    }

    #[test]
    fn child_span_links_to_parent() {
        let root = Span::root("root", 100);
        let child =
            Span::child("child", root.span_id.clone(), root.trace_id.clone(), 200);
        assert_eq!(child.parent_id.as_ref(), Some(&root.span_id));
        assert_eq!(child.trace_id, root.trace_id);
    }

    #[test]
    fn span_duration() {
        let mut s = Span::root("dur", 1000);
        s.finish(2000, SpanStatus::Ok);
        assert_eq!(s.duration_ns(), Some(1000));
    }
}
