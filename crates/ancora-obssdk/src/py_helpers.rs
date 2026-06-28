/// Python SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Simulates the Python SDK's trace accessor interface.
pub struct PyTraceAccessor {
    trace: Trace,
}

impl PyTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        PyTraceAccessor {
            trace: Trace::new(trace_id),
        }
    }

    /// Record a span in the Python SDK style (keyword-argument-like).
    pub fn record_span(
        &mut self,
        span_id: impl Into<String>,
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
    ) {
        let span = Span::new(span_id, name, start_ns).finish(end_ns);
        self.trace.add_span(span);
    }

    pub fn record_child_span(
        &mut self,
        span_id: impl Into<String>,
        parent_id: impl Into<String>,
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
    ) {
        let span = Span::new(span_id, name, start_ns)
            .with_parent(parent_id)
            .finish(end_ns);
        self.trace.add_span(span);
    }

    pub fn spans(&self) -> &[Span] {
        &self.trace.spans
    }

    pub fn trace_id(&self) -> &str {
        &self.trace.trace_id
    }

    pub fn span_count(&self) -> usize {
        self.trace.span_count()
    }

    /// Python-style dict export for the trace.
    pub fn to_dict(&self) -> Vec<(String, String)> {
        self.trace
            .spans
            .iter()
            .map(|s| (s.span_id.clone(), s.name.clone()))
            .collect()
    }
}

/// Python SDK cost accessor.
pub struct PyCostAccessor {
    records: Vec<CostRecord>,
}

impl PyCostAccessor {
    pub fn new() -> Self {
        PyCostAccessor { records: Vec::new() }
    }

    pub fn record(&mut self, cost: CostRecord) {
        self.records.push(cost);
    }

    pub fn total_tokens(&self) -> u64 {
        self.records.iter().map(|r| r.total_tokens()).sum()
    }

    pub fn records(&self) -> &[CostRecord] {
        &self.records
    }

    pub fn summarize(&self) -> String {
        format!(
            "total_tokens={} records={}",
            self.total_tokens(),
            self.records.len()
        )
    }
}

impl Default for PyCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn py_trace_accessor_spans() {
        let mut acc = PyTraceAccessor::new("py-trace-1");
        acc.record_span("s1", "llm.call", 0, 500);
        acc.record_child_span("s2", "s1", "embed", 10, 100);
        assert_eq!(acc.span_count(), 2);
        let dict = acc.to_dict();
        assert_eq!(dict.len(), 2);
    }

    #[test]
    fn py_cost_summarize() {
        let mut acc = PyCostAccessor::new();
        acc.record(CostRecord::new("py-trace-1", 300, 150, "claude-3"));
        assert!(acc.summarize().contains("450"));
    }
}
