/// Go SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Simulates the Go SDK's trace accessor interface.
pub struct GoTraceAccessor {
    trace: Trace,
}

impl GoTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        GoTraceAccessor {
            trace: Trace::new(trace_id),
        }
    }

    /// Record a span as the Go SDK would emit it.
    pub fn record_span(&mut self, span_id: impl Into<String>, name: impl Into<String>, start_ns: u64, end_ns: u64) {
        let span = Span::new(span_id, name, start_ns).finish(end_ns);
        self.trace.add_span(span);
    }

    /// Record a child span.
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
}

/// Go SDK cost accessor.
pub struct GoCostAccessor {
    records: Vec<CostRecord>,
}

impl GoCostAccessor {
    pub fn new() -> Self {
        GoCostAccessor { records: Vec::new() }
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
}

impl Default for GoCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_trace_accessor_basic() {
        let mut acc = GoTraceAccessor::new("go-trace-1");
        acc.record_span("s1", "http.handler", 0, 100);
        assert_eq!(acc.span_count(), 1);
        assert_eq!(acc.trace_id(), "go-trace-1");
    }

    #[test]
    fn go_cost_accessor_total() {
        let mut acc = GoCostAccessor::new();
        acc.record(CostRecord::new("go-trace-1", 200, 100, "claude-3"));
        assert_eq!(acc.total_tokens(), 300);
    }
}
