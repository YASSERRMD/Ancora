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

impl SpanEmitter for InMemoryExporter {
    fn emit(&self, span: Span) {
        self.spans.lock().unwrap().push(span);
    }
}
