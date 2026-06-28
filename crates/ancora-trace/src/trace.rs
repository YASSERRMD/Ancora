/// Unified trace model built from journal events.
///
/// A `Trace` owns the root span and all child spans that together
/// represent one logical run of an agent or tool pipeline.

use std::collections::HashMap;

use crate::span::{Span, SpanId, TraceId};

/// The complete trace tree for a single agent run.
#[derive(Debug, Clone)]
pub struct Trace {
    /// Globally-unique identifier for this trace.
    pub trace_id: TraceId,
    /// All spans keyed by their span-id.
    spans: HashMap<SpanId, Span>,
    /// Id of the root span (no parent).
    pub root_id: SpanId,
}

impl Trace {
    /// Create a new trace with a root span.
    pub fn new(trace_id: TraceId, root: Span) -> Self {
        let root_id = root.span_id.clone();
        let mut spans = HashMap::new();
        spans.insert(root_id.clone(), root);
        Trace { trace_id, spans, root_id }
    }

    /// Insert a child span.  Returns an error if the parent is unknown.
    pub fn add_span(&mut self, span: Span) -> Result<(), TraceError> {
        if let Some(ref pid) = span.parent_id {
            if !self.spans.contains_key(pid) {
                return Err(TraceError::UnknownParent(pid.clone()));
            }
        }
        self.spans.insert(span.span_id.clone(), span);
        Ok(())
    }

    /// Retrieve a span by id.
    pub fn get_span(&self, id: &SpanId) -> Option<&Span> {
        self.spans.get(id)
    }

    /// All spans in insertion order (deterministic for replay).
    pub fn all_spans(&self) -> Vec<&Span> {
        let mut v: Vec<&Span> = self.spans.values().collect();
        // Sort by start_ns for reproducibility.
        v.sort_by_key(|s| s.start_ns);
        v
    }

    /// Direct children of a span.
    pub fn children_of(&self, parent: &SpanId) -> Vec<&Span> {
        let mut ch: Vec<&Span> = self.spans
            .values()
            .filter(|s| s.parent_id.as_ref() == Some(parent))
            .collect();
        ch.sort_by_key(|s| s.start_ns);
        ch
    }

    /// Total span count.
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }
}

/// Errors that can occur while building a trace.
#[derive(Debug, Clone, PartialEq)]
pub enum TraceError {
    UnknownParent(SpanId),
    DuplicateSpanId(SpanId),
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceError::UnknownParent(id) => write!(f, "unknown parent span: {}", id.0),
            TraceError::DuplicateSpanId(id) => write!(f, "duplicate span id: {}", id.0),
        }
    }
}

/// Build a `Trace` from a flat list of journal-sourced spans.
///
/// The first span without a parent_id is used as the root.
pub fn build_trace_from_spans(spans: Vec<Span>) -> Result<Trace, TraceError> {
    if spans.is_empty() {
        let root = Span::root("empty-trace", 0);
        return Ok(Trace::new(root.trace_id.clone(), root));
    }

    let root = spans
        .iter()
        .find(|s| s.parent_id.is_none())
        .cloned()
        .unwrap_or_else(|| spans[0].clone());

    let trace_id = root.trace_id.clone();
    let mut trace = Trace::new(trace_id, root.clone());

    for span in spans {
        if span.span_id == root.span_id {
            continue;
        }
        trace.add_span(span)?;
    }
    Ok(trace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn trace_root_reachable() {
        let root = Span::root("run-1", 1_000);
        let tid = root.trace_id.clone();
        let trace = Trace::new(tid, root.clone());
        assert_eq!(trace.root_id, root.span_id);
    }

    #[test]
    fn add_span_unknown_parent_fails() {
        let root = Span::root("run-1", 0);
        let tid = root.trace_id.clone();
        let mut trace = Trace::new(tid.clone(), root);
        let bad_parent = SpanId("ghost".into());
        let orphan = Span::child("orphan", bad_parent.clone(), tid, 100);
        let err = trace.add_span(orphan).unwrap_err();
        assert_eq!(err, TraceError::UnknownParent(bad_parent));
    }
}
