/// Rust SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Rust-native trace accessor using builder-pattern ergonomics.
pub struct RsTraceAccessor {
    trace: Trace,
}

impl RsTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        RsTraceAccessor {
            trace: Trace::new(trace_id),
        }
    }

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

    /// Return the root span duration if available.
    pub fn root_duration_ns(&self) -> Option<u64> {
        self.trace.root_span().and_then(|s| s.duration_ns())
    }
}

/// Rust SDK cost accessor.
pub struct RsCostAccessor {
    records: Vec<CostRecord>,
}

impl RsCostAccessor {
    pub fn new() -> Self {
        RsCostAccessor {
            records: Vec::new(),
        }
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

    pub fn iter_models(&self) -> impl Iterator<Item = &str> {
        self.records.iter().map(|r| r.model.as_str())
    }
}

impl Default for RsCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_root_duration() {
        let mut acc = RsTraceAccessor::new("rs-trace-1");
        acc.record_span("s1", "agent.run", 0, 999);
        assert_eq!(acc.root_duration_ns(), Some(999));
    }

    #[test]
    fn rs_cost_models() {
        let mut acc = RsCostAccessor::new();
        acc.record(CostRecord::new("rs-t1", 100, 50, "claude-3"));
        let models: Vec<&str> = acc.iter_models().collect();
        assert_eq!(models, vec!["claude-3"]);
    }
}
