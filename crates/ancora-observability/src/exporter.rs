use std::sync::{Arc, Mutex};

use crate::span::Span;

/// Receives and records spans.
pub trait SpanEmitter: Send + Sync {
    fn emit(&self, span: Span);
    fn flush(&self) {}
}

/// In-memory exporter for testing; captures all emitted spans.
#[derive(Default)]
pub struct InMemoryExporter {
    spans: Arc<Mutex<Vec<Span>>>,
}

impl InMemoryExporter {
    pub fn new() -> Self {
        Self { spans: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn spans(&self) -> Vec<Span> {
        self.spans.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.spans.lock().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.spans.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.lock().unwrap().is_empty()
    }
}

/// No-op emitter that discards all spans.
pub struct NoopEmitter;

impl SpanEmitter for NoopEmitter {
    fn emit(&self, _span: Span) {}
}

/// Fan-out emitter that forwards to multiple backends.
pub struct MultiEmitter {
    emitters: Vec<Box<dyn SpanEmitter>>,
}

impl MultiEmitter {
    pub fn new(emitters: Vec<Box<dyn SpanEmitter>>) -> Self {
        Self { emitters }
    }
}

impl SpanEmitter for MultiEmitter {
    fn emit(&self, span: Span) {
        for e in &self.emitters {
            e.emit(span.clone());
        }
    }
}

impl SpanEmitter for InMemoryExporter {
    fn emit(&self, span: Span) {
        self.spans.lock().unwrap().push(span);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::SpanValue;

    #[test]
    fn exporter_collects_emitted_spans() {
        let exp = InMemoryExporter::new();
        exp.emit(Span::new("op1"));
        exp.emit(Span::new("op2"));
        assert_eq!(exp.len(), 2);
        assert_eq!(exp.spans()[0].name, "op1");
        assert_eq!(exp.spans()[1].name, "op2");
    }

    #[test]
    fn exporter_clear_empties_spans() {
        let exp = InMemoryExporter::new();
        exp.emit(Span::new("op"));
        assert!(!exp.is_empty());
        exp.clear();
        assert!(exp.is_empty());
    }

    #[test]
    fn exporter_preserves_span_attributes() {
        let exp = InMemoryExporter::new();
        exp.emit(Span::new("op").set("k", "v"));
        let spans = exp.spans();
        assert_eq!(spans[0].get("k"), Some(&SpanValue::String("v".into())));
    }

    #[test]
    fn noop_emitter_discards_spans() {
        let noop = NoopEmitter;
        noop.emit(Span::new("discarded"));
    }

    #[test]
    fn multi_emitter_fans_out_to_all_backends() {
        let a = Arc::new(InMemoryExporter::new());
        let b = Arc::new(InMemoryExporter::new());
        let a2 = a.clone();
        let b2 = b.clone();
        let multi = MultiEmitter::new(vec![
            Box::new(InMemoryExporter { spans: a2.spans.clone() }),
            Box::new(InMemoryExporter { spans: b2.spans.clone() }),
        ]);
        multi.emit(Span::new("fanned"));
        assert_eq!(a.len(), 1);
        assert_eq!(b.len(), 1);
    }
}
