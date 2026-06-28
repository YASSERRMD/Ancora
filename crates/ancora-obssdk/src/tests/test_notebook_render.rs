use crate::context::{Span, Trace};
use crate::notebook::{NotebookOutputFormat, NotebookTraceRenderer};

fn make_trace() -> Trace {
    let mut t = Trace::new("nb-test-trace");
    t.add_span(Span::new("s1", "root.op", 0).finish(2000));
    t.add_span(Span::new("s2", "sub.op", 200).with_parent("s1").finish(1800));
    t
}

#[test]
fn notebook_plain_render_contains_trace_id() {
    let renderer = NotebookTraceRenderer::new();
    let trace = make_trace();
    let output = renderer.render_plain(&trace);
    assert_eq!(output.format, NotebookOutputFormat::PlainText);
    assert!(output.content.contains("nb-test-trace"));
    assert!(!output.is_empty());
}

#[test]
fn notebook_html_render_has_table() {
    let renderer = NotebookTraceRenderer::new();
    let trace = make_trace();
    let output = renderer.render_html(&trace);
    assert_eq!(output.format, NotebookOutputFormat::Html);
    assert!(output.content.contains("<table>"));
    assert!(output.content.contains("root.op"));
    assert!(output.content.contains("sub.op"));
}

#[test]
fn notebook_markdown_render_has_header() {
    let renderer = NotebookTraceRenderer::new();
    let trace = make_trace();
    let output = renderer.render_markdown(&trace);
    assert_eq!(output.format, NotebookOutputFormat::Markdown);
    assert!(output.content.contains("## Trace"));
    assert!(output.content.contains("nb-test-trace"));
}

#[test]
fn notebook_render_duration_shown() {
    let renderer = NotebookTraceRenderer::new();
    let trace = make_trace();
    let output = renderer.render_plain(&trace);
    assert!(output.content.contains("ns"));
}
