/// TypeScript SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Simulates the TypeScript/Node SDK's trace accessor interface.
pub struct TsTraceAccessor {
    trace: Trace,
}

impl TsTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        TsTraceAccessor {
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

    /// TypeScript-style JSON-like serialization of spans.
    pub fn to_json_strings(&self) -> Vec<String> {
        self.trace
            .spans
            .iter()
            .map(|s| {
                format!(
                    r#"{{"spanId":"{}","name":"{}","startNs":{}}}"#,
                    s.span_id, s.name, s.start_ns
                )
            })
            .collect()
    }
}

/// TypeScript SDK cost accessor.
pub struct TsCostAccessor {
    records: Vec<CostRecord>,
}

impl TsCostAccessor {
    pub fn new() -> Self {
        TsCostAccessor {
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
}

impl Default for TsCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ts_trace_json_output() {
        let mut acc = TsTraceAccessor::new("ts-trace-1");
        acc.record_span("s1", "fetch", 1000, 2000);
        let json_strs = acc.to_json_strings();
        assert_eq!(json_strs.len(), 1);
        assert!(json_strs[0].contains("fetch"));
    }

    #[test]
    fn ts_cost_accessor() {
        let mut acc = TsCostAccessor::new();
        acc.record(CostRecord::new("ts-trace-1", 100, 50, "claude-3"));
        assert_eq!(acc.total_tokens(), 150);
    }
}
