/// Notebook render path - inline trace visualization for Jupyter-style outputs.
use crate::context::Trace;

/// Output format for notebook cells.
#[derive(Debug, Clone, PartialEq)]
pub enum NotebookOutputFormat {
    PlainText,
    Html,
    Markdown,
}

/// A rendered cell output for a notebook.
#[derive(Debug, Clone)]
pub struct NotebookCellOutput {
    pub format: NotebookOutputFormat,
    pub content: String,
}

impl NotebookCellOutput {
    pub fn new(format: NotebookOutputFormat, content: impl Into<String>) -> Self {
        NotebookCellOutput {
            format,
            content: content.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Renders a trace into notebook-compatible output.
pub struct NotebookTraceRenderer;

impl NotebookTraceRenderer {
    pub fn new() -> Self {
        NotebookTraceRenderer
    }

    /// Render trace as plain text (ASCII table style).
    pub fn render_plain(&self, trace: &Trace) -> NotebookCellOutput {
        let mut lines = Vec::new();
        lines.push(format!("Trace: {}", trace.trace_id));
        lines.push(format!("Spans: {}", trace.spans.len()));
        lines.push(String::from("---"));
        for span in &trace.spans {
            let parent = span.parent_id.as_deref().unwrap_or("(root)");
            let dur = span
                .duration_ns()
                .map(|d| format!("{}ns", d))
                .unwrap_or_else(|| "open".into());
            lines.push(format!(
                "  [{parent}] {name} | {dur}",
                parent = parent,
                name = span.name,
                dur = dur
            ));
        }
        NotebookCellOutput::new(NotebookOutputFormat::PlainText, lines.join("\n"))
    }

    /// Render trace as HTML table.
    pub fn render_html(&self, trace: &Trace) -> NotebookCellOutput {
        let mut rows = String::new();
        for span in &trace.spans {
            let parent = span.parent_id.as_deref().unwrap_or("(root)");
            let dur = span
                .duration_ns()
                .map(|d| format!("{}ns", d))
                .unwrap_or_else(|| "open".into());
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                span.span_id, span.name, parent, dur
            ));
        }
        let html = format!(
            "<table><thead><tr><th>ID</th><th>Name</th><th>Parent</th><th>Duration</th></tr></thead><tbody>{}</tbody></table>",
            rows
        );
        NotebookCellOutput::new(NotebookOutputFormat::Html, html)
    }

    /// Render trace as Markdown.
    pub fn render_markdown(&self, trace: &Trace) -> NotebookCellOutput {
        let mut lines = Vec::new();
        lines.push(format!("## Trace `{}`", trace.trace_id));
        lines.push(String::new());
        lines.push("| ID | Name | Parent | Duration |".into());
        lines.push("|----|------|--------|----------|".into());
        for span in &trace.spans {
            let parent = span.parent_id.as_deref().unwrap_or("(root)");
            let dur = span
                .duration_ns()
                .map(|d| format!("{}ns", d))
                .unwrap_or_else(|| "open".into());
            lines.push(format!(
                "| {} | {} | {} | {} |",
                span.span_id, span.name, parent, dur
            ));
        }
        NotebookCellOutput::new(NotebookOutputFormat::Markdown, lines.join("\n"))
    }
}

impl Default for NotebookTraceRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Span;

    fn make_trace() -> Trace {
        let mut t = Trace::new("nb-trace-1");
        t.add_span(Span::new("s1", "root.call", 0).finish(1000));
        t.add_span(
            Span::new("s2", "child.call", 100)
                .with_parent("s1")
                .finish(900),
        );
        t
    }

    #[test]
    fn notebook_plain_render() {
        let renderer = NotebookTraceRenderer::new();
        let trace = make_trace();
        let output = renderer.render_plain(&trace);
        assert_eq!(output.format, NotebookOutputFormat::PlainText);
        assert!(output.content.contains("nb-trace-1"));
        assert!(output.content.contains("root.call"));
    }

    #[test]
    fn notebook_html_render() {
        let renderer = NotebookTraceRenderer::new();
        let trace = make_trace();
        let output = renderer.render_html(&trace);
        assert_eq!(output.format, NotebookOutputFormat::Html);
        assert!(output.content.contains("<table>"));
        assert!(output.content.contains("child.call"));
    }

    #[test]
    fn notebook_markdown_render() {
        let renderer = NotebookTraceRenderer::new();
        let trace = make_trace();
        let output = renderer.render_markdown(&trace);
        assert_eq!(output.format, NotebookOutputFormat::Markdown);
        assert!(output.content.contains("## Trace"));
    }
}
