/// Trace and cost emission for sample applications.
///
/// Each app records spans and associated token costs so that
/// operators can audit usage after an offline run.

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Span {
    pub name: String,
    pub app: String,
    pub duration_ms: u64,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub cost_usd: f64,
}

impl Span {
    /// Cost per 1k tokens (stub rate for offline testing).
    const COST_PER_1K: f64 = 0.002;

    pub fn new(
        name: impl Into<String>,
        app: impl Into<String>,
        duration_ms: u64,
        input_tokens: usize,
        output_tokens: usize,
    ) -> Self {
        let total_tokens = input_tokens + output_tokens;
        let cost_usd = (total_tokens as f64 / 1000.0) * Self::COST_PER_1K;
        Self {
            name: name.into(),
            app: app.into(),
            duration_ms,
            input_tokens,
            output_tokens,
            cost_usd,
        }
    }
}

#[derive(Debug, Default)]
pub struct Tracer {
    spans: Vec<Span>,
}

impl Tracer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.spans.iter().map(|s| s.cost_usd).sum()
    }

    pub fn total_tokens(&self) -> usize {
        self.spans
            .iter()
            .map(|s| s.input_tokens + s.output_tokens)
            .sum()
    }

    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    pub fn spans_for_app(&self, app: &str) -> Vec<&Span> {
        self.spans.iter().filter(|s| s.app == app).collect()
    }
}

/// RAII guard that records a span when dropped.
pub struct SpanGuard<'a> {
    tracer: &'a mut Tracer,
    name: String,
    app: String,
    start: Instant,
    input_tokens: usize,
    output_tokens: usize,
}

impl<'a> SpanGuard<'a> {
    pub fn start(
        tracer: &'a mut Tracer,
        name: impl Into<String>,
        app: impl Into<String>,
        input_tokens: usize,
        output_tokens: usize,
    ) -> Self {
        Self {
            tracer,
            name: name.into(),
            app: app.into(),
            start: Instant::now(),
            input_tokens,
            output_tokens,
        }
    }
}

impl Drop for SpanGuard<'_> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let ms = elapsed.as_millis() as u64;
        self.tracer.record(Span::new(
            self.name.clone(),
            self.app.clone(),
            ms,
            self.input_tokens,
            self.output_tokens,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracer_accumulates_cost() {
        let mut tracer = Tracer::new();
        tracer.record(Span::new("qa", "document-qa", 5, 100, 50));
        tracer.record(Span::new("research", "research-assistant", 8, 200, 80));
        assert_eq!(tracer.span_count(), 2);
        assert!(tracer.total_cost_usd() > 0.0);
        assert_eq!(tracer.total_tokens(), 430);
    }
}
