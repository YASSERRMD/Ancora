/// Java SDK observability helpers - trace and cost accessor facades.
use crate::context::{CostRecord, Span, Trace};

/// Simulates the Java SDK's trace accessor interface (OpenTelemetry-style).
pub struct JavaTraceAccessor {
    trace: Trace,
}

impl JavaTraceAccessor {
    pub fn new(trace_id: impl Into<String>) -> Self {
        JavaTraceAccessor {
            trace: Trace::new(trace_id),
        }
    }

    /// Start a span in Java OTel style.
    pub fn start_span(
        &mut self,
        span_id: impl Into<String>,
        name: impl Into<String>,
        start_ns: u64,
        end_ns: u64,
    ) {
        let span = Span::new(span_id, name, start_ns).finish(end_ns);
        self.trace.add_span(span);
    }

    pub fn start_child_span(
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

    /// Export span names as would appear in a Java log.
    pub fn span_names(&self) -> Vec<&str> {
        self.trace.spans.iter().map(|s| s.name.as_str()).collect()
    }
}

/// Java SDK cost accessor.
pub struct JavaCostAccessor {
    records: Vec<CostRecord>,
}

impl JavaCostAccessor {
    pub fn new() -> Self {
        JavaCostAccessor {
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

    /// Java-style toString for cost summary (name matches the Java-side
    /// `toString()` convention this JNI-facing type mirrors).
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!(
            "JavaCostAccessor{{totalTokens={}, records={}}}",
            self.total_tokens(),
            self.records.len()
        )
    }
}

impl Default for JavaCostAccessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_span_names() {
        let mut acc = JavaTraceAccessor::new("java-trace-1");
        acc.start_span("s1", "Servlet.service", 0, 100);
        acc.start_child_span("s2", "s1", "JDBC.query", 10, 80);
        let names = acc.span_names();
        assert!(names.contains(&"JDBC.query"));
    }

    #[test]
    fn java_cost_to_string() {
        let mut acc = JavaCostAccessor::new();
        acc.record(CostRecord::new("java-t1", 500, 250, "claude-3"));
        assert!(acc.to_string().contains("750"));
    }
}
