/// Trace context shared across all SDK language helpers.
use std::collections::HashMap;

/// A single span within a trace.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub span_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub start_ns: u64,
    pub end_ns: Option<u64>,
    pub attributes: HashMap<String, String>,
}

impl Span {
    pub fn new(span_id: impl Into<String>, name: impl Into<String>, start_ns: u64) -> Self {
        Span {
            span_id: span_id.into(),
            parent_id: None,
            name: name.into(),
            start_ns,
            end_ns: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn finish(mut self, end_ns: u64) -> Self {
        self.end_ns = Some(end_ns);
        self
    }

    pub fn set_attr(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    pub fn duration_ns(&self) -> Option<u64> {
        self.end_ns.map(|e| e.saturating_sub(self.start_ns))
    }
}

/// A complete trace holding multiple spans.
#[derive(Debug, Clone)]
pub struct Trace {
    pub trace_id: String,
    pub spans: Vec<Span>,
}

impl Trace {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Trace {
            trace_id: trace_id.into(),
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn root_span(&self) -> Option<&Span> {
        self.spans.iter().find(|s| s.parent_id.is_none())
    }

    pub fn span_count(&self) -> usize {
        self.spans.len()
    }
}

/// Cost record associated with a trace.
#[derive(Debug, Clone, PartialEq)]
pub struct CostRecord {
    pub trace_id: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub model: String,
}

impl CostRecord {
    pub fn new(
        trace_id: impl Into<String>,
        input_tokens: u64,
        output_tokens: u64,
        model: impl Into<String>,
    ) -> Self {
        CostRecord {
            trace_id: trace_id.into(),
            input_tokens,
            output_tokens,
            model: model.into(),
        }
    }

    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }
}

/// Eval result from running an evaluation against a trace.
#[derive(Debug, Clone, PartialEq)]
pub struct EvalResult {
    pub trace_id: String,
    pub passed: bool,
    pub score: f64,
    pub notes: String,
}

impl EvalResult {
    pub fn new(trace_id: impl Into<String>, passed: bool, score: f64, notes: impl Into<String>) -> Self {
        EvalResult {
            trace_id: trace_id.into(),
            passed,
            score,
            notes: notes.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_duration() {
        let span = Span::new("s1", "root", 100).finish(200);
        assert_eq!(span.duration_ns(), Some(100));
    }

    #[test]
    fn trace_root_span() {
        let mut trace = Trace::new("t1");
        trace.add_span(Span::new("s1", "root", 0).finish(10));
        trace.add_span(Span::new("s2", "child", 2).with_parent("s1").finish(8));
        assert_eq!(trace.root_span().unwrap().span_id, "s1");
        assert_eq!(trace.span_count(), 2);
    }

    #[test]
    fn cost_record_total() {
        let cost = CostRecord::new("t1", 100, 50, "claude-3");
        assert_eq!(cost.total_tokens(), 150);
    }
}
