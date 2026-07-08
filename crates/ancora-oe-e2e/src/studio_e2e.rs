/// Studio module: renders a run end-to-end for display in the observability studio.
use crate::trace_e2e::Trace;

/// A rendered view of a run for studio display.
#[derive(Debug, Clone)]
pub struct StudioRunView {
    pub run_id: String,
    pub trace_summary: TraceSummary,
    pub span_rows: Vec<SpanRow>,
}

/// Summary of a trace for display.
#[derive(Debug, Clone)]
pub struct TraceSummary {
    pub total_spans: usize,
    pub duration_ms: f64,
    pub root_name: String,
}

/// A single span rendered as a row.
#[derive(Debug, Clone)]
pub struct SpanRow {
    pub span_id: String,
    pub name: String,
    pub depth: usize,
    pub start_ms: f64,
    pub end_ms: f64,
}

impl SpanRow {
    pub fn duration_ms(&self) -> f64 {
        self.end_ms - self.start_ms
    }
}

/// Renders a trace into a studio view.
pub fn render_trace(run_id: &str, trace: &Trace) -> StudioRunView {
    let root_name = trace
        .root_span()
        .map(|s| s.name.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let duration_ms = trace.total_duration_ns() as f64 / 1_000_000.0;

    let summary = TraceSummary {
        total_spans: trace.spans.len(),
        duration_ms,
        root_name,
    };

    let span_rows = trace
        .spans
        .iter()
        .map(|s| {
            let depth = if s.parent_id.is_none() { 0 } else { 1 };
            SpanRow {
                span_id: s.span_id.clone(),
                name: s.name.clone(),
                depth,
                start_ms: s.start_ns as f64 / 1_000_000.0,
                end_ms: s.end_ns as f64 / 1_000_000.0,
            }
        })
        .collect();

    StudioRunView {
        run_id: run_id.to_string(),
        trace_summary: summary,
        span_rows,
    }
}

/// Formats a studio view as a plain-text report (no external deps).
pub fn format_view(view: &StudioRunView) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Run: {}", view.run_id));
    lines.push(format!(
        "Trace: {} spans, {:.2} ms, root={}",
        view.trace_summary.total_spans,
        view.trace_summary.duration_ms,
        view.trace_summary.root_name,
    ));
    for row in &view.span_rows {
        let indent = "  ".repeat(row.depth);
        lines.push(format!(
            "{}[{}] {} ({:.2} ms)",
            indent,
            row.span_id,
            row.name,
            row.duration_ms(),
        ));
    }
    lines.join("\n")
}
