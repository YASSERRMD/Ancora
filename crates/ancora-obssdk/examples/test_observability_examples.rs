/// Integration test: runs all per-language observability accessors end-to-end.
///
/// Run with: cargo run --example test_observability_examples -p ancora-obssdk
use ancora_obssdk::context::{Span, Trace};
use ancora_obssdk::dotnet_helpers::DotnetTraceAccessor;
use ancora_obssdk::eval_helpers::{EvalCriteria, EvalRunner};
use ancora_obssdk::go_helpers::GoTraceAccessor;
use ancora_obssdk::java_helpers::JavaTraceAccessor;
use ancora_obssdk::notebook::NotebookTraceRenderer;
use ancora_obssdk::py_helpers::PyTraceAccessor;
use ancora_obssdk::rs_helpers::RsTraceAccessor;
use ancora_obssdk::ts_helpers::TsTraceAccessor;

fn check(label: &str, ok: bool) {
    if ok {
        println!("PASS: {}", label);
    } else {
        eprintln!("FAIL: {}", label);
        std::process::exit(1);
    }
}

fn main() {
    println!("=== observability examples test run ===\n");

    // Go
    {
        let mut acc = GoTraceAccessor::new("go-ex-t1");
        acc.record_span("s1", "grpc.call", 0, 1000);
        check("go: span count == 1", acc.span_count() == 1);
        check("go: trace id correct", acc.trace_id() == "go-ex-t1");
    }

    // Python
    {
        let mut acc = PyTraceAccessor::new("py-ex-t1");
        acc.record_span("s1", "llm.invoke", 0, 500);
        acc.record_child_span("s2", "s1", "embed", 50, 300);
        check("py: span count == 2", acc.span_count() == 2);
        check("py: dict len == 2", acc.to_dict().len() == 2);
    }

    // TypeScript
    {
        let mut acc = TsTraceAccessor::new("ts-ex-t1");
        acc.record_span("s1", "api.call", 0, 800);
        let json = acc.to_json_strings();
        check("ts: json output non-empty", !json.is_empty());
        check("ts: json contains span name", json[0].contains("api.call"));
    }

    // .NET
    {
        let mut acc = DotnetTraceAccessor::new("dotnet-ex-t1");
        acc.start_activity("s1", "Controller.Action", 0, 2000);
        check(
            "dotnet: traceparent prefix",
            acc.traceparent().starts_with("00-"),
        );
        check("dotnet: span count == 1", acc.span_count() == 1);
    }

    // Java
    {
        let mut acc = JavaTraceAccessor::new("java-ex-t1");
        acc.start_span("s1", "Service.handle", 0, 3000);
        let names = acc.span_names();
        check("java: span name found", names.contains(&"Service.handle"));
    }

    // Rust
    {
        let mut acc = RsTraceAccessor::new("rs-ex-t1");
        acc.record_span("s1", "handler", 0, 750);
        check("rust: root duration", acc.root_duration_ns() == Some(750));
    }

    // Notebook render
    {
        let mut trace = Trace::new("nb-ex-t1");
        trace.add_span(Span::new("s1", "root", 0).finish(1000));
        let renderer = NotebookTraceRenderer::new();
        let plain = renderer.render_plain(&trace);
        let html = renderer.render_html(&trace);
        let md = renderer.render_markdown(&trace);
        check("notebook: plain non-empty", !plain.is_empty());
        check("notebook: html has table", html.content.contains("<table>"));
        check("notebook: md has header", md.content.contains("## Trace"));
    }

    // Eval
    {
        let mut trace = Trace::new("eval-ex-t1");
        trace.add_span(Span::new("s1", "agent.run", 0).finish(500));
        let criteria = EvalCriteria::new("basic")
            .with_min_spans(1)
            .with_required_span("agent.run");
        let result = EvalRunner::new().evaluate(&trace, &criteria);
        check("eval: passed", result.passed);
        check("eval: score 1.0", (result.score - 1.0).abs() < f64::EPSILON);
    }

    println!("\n=== all checks passed ===");
}
