/// .NET SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Simulates the .NET SDK's trace accessor interface (Activity-style).
pub struct DotnetTraceAccessor {
    trace: Trace,
}

impl DotnetTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        DotnetTraceAccessor {
            trace: Trace::new(trace_id),
        }
    }

    /// Record an Activity (span) in .NET style.
    pub fn start_activity(
        &mut self,
        span_id: impl Into<String>,
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
    ) {
        let span = Span::new(span_id, name, start_ns).finish(end_ns);
        self.trace.add_span(span);
    }

    pub fn start_child_activity(
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

    /// Export in W3C TraceContext header format (simplified).
    pub fn traceparent(&self) -> String {
        format!("00-{}-0000000000000001-01", self.trace.trace_id)
    }
}

/// .NET SDK cost accessor.
pub struct DotnetCostAccessor {
    records: Vec<CostRecord>,
}

impl DotnetCostAccessor {
    pub fn new() -> Self {
        DotnetCostAccessor { records: Vec::new() }
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

impl Default for DotnetCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotnet_traceparent_format() {
        let mut acc = DotnetTraceAccessor::new("abc123");
        acc.start_activity("s1", "GraphQL.Query", 0, 100);
        let tp = acc.traceparent();
        assert!(tp.starts_with("00-abc123-"));
    }

    #[test]
    fn dotnet_cost_accessor() {
        let mut acc = DotnetCostAccessor::new();
        acc.record(CostRecord::new("dotnet-t1", 400, 200, "claude-3"));
        assert_eq!(acc.total_tokens(), 600);
    }
}
