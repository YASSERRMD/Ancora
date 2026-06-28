/// End-to-end trace module for observability testing.
/// Provides structures and logic for recording, exporting, and verifying traces.

use std::collections::HashMap;

/// A single span within a trace.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub span_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub attributes: HashMap<String, String>,
}

impl Span {
    pub fn new(span_id: impl Into<String>, name: impl Into<String>, start_ns: u64, end_ns: u64) -> Self {
        Self {
            span_id: span_id.into(),
            parent_id: None,
            name: name.into(),
            start_ns,
            end_ns,
            attributes: HashMap::new(),
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn duration_ns(&self) -> u64 {
        self.end_ns.saturating_sub(self.start_ns)
    }
}

/// A complete trace composed of multiple spans.
#[derive(Debug, Clone)]
pub struct Trace {
    pub trace_id: String,
    pub spans: Vec<Span>,
}

impl Trace {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn is_complete(&self) -> bool {
        !self.spans.is_empty()
    }

    pub fn root_span(&self) -> Option<&Span> {
        self.spans.iter().find(|s| s.parent_id.is_none())
    }

    pub fn child_spans(&self, parent_id: &str) -> Vec<&Span> {
        self.spans
            .iter()
            .filter(|s| s.parent_id.as_deref() == Some(parent_id))
            .collect()
    }

    pub fn total_duration_ns(&self) -> u64 {
        self.root_span().map(|s| s.duration_ns()).unwrap_or(0)
    }
}

/// Trait for exporting traces to a collector.
pub trait TraceExporter {
    fn export(&mut self, trace: &Trace) -> Result<(), String>;
}

/// A mock in-memory trace collector for testing.
#[derive(Debug, Default)]
pub struct MockCollector {
    pub traces: Vec<Trace>,
}

impl TraceExporter for MockCollector {
    fn export(&mut self, trace: &Trace) -> Result<(), String> {
        if trace.trace_id.is_empty() {
            return Err("trace_id cannot be empty".to_string());
        }
        self.traces.push(trace.clone());
        Ok(())
    }
}

impl MockCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.traces.len()
    }

    pub fn find_trace(&self, trace_id: &str) -> Option<&Trace> {
        self.traces.iter().find(|t| t.trace_id == trace_id)
    }
}

/// Builds a simple agent run trace for testing.
pub fn build_run_trace(trace_id: &str) -> Trace {
    let mut trace = Trace::new(trace_id);

    let root = Span::new("span-root", "agent.run", 0, 1_000_000)
        .with_attribute("agent.name", "test-agent")
        .with_attribute("run.id", trace_id);

    let child1 = Span::new("span-llm", "llm.invoke", 100, 800_000)
        .with_parent("span-root")
        .with_attribute("model", "local-judge");

    let child2 = Span::new("span-tool", "tool.call", 820_000, 950_000)
        .with_parent("span-root")
        .with_attribute("tool.name", "search");

    trace.add_span(root);
    trace.add_span(child1);
    trace.add_span(child2);
    trace
}
