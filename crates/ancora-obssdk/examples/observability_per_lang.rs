/// Example: Observability helpers for each language SDK.
///
/// Run with: cargo run --example observability_per_lang -p ancora-obssdk

use ancora_obssdk::context::{CostRecord, Span, Trace};
use ancora_obssdk::eval_helpers::{EvalCriteria, EvalRunner, run_multilang_eval};
use ancora_obssdk::go_helpers::{GoCostAccessor, GoTraceAccessor};
use ancora_obssdk::py_helpers::{PyCostAccessor, PyTraceAccessor};
use ancora_obssdk::ts_helpers::{TsCostAccessor, TsTraceAccessor};
use ancora_obssdk::dotnet_helpers::{DotnetCostAccessor, DotnetTraceAccessor};
use ancora_obssdk::java_helpers::{JavaCostAccessor, JavaTraceAccessor};
use ancora_obssdk::rs_helpers::{RsCostAccessor, RsTraceAccessor};
use ancora_obssdk::notebook::NotebookTraceRenderer;

fn make_trace(id: &str) -> Trace {
    let mut t = Trace::new(id);
    t.add_span(Span::new("s1", "agent.run", 0).finish(10_000));
    t.add_span(Span::new("s2", "llm.call", 100).with_parent("s1").finish(9_000));
    t
}

fn main() {
    println!("=== ancora-obssdk: observability per language ===\n");

    // Go
    {
        let mut acc = GoTraceAccessor::new("go-trace-001");
        acc.record_span("s1", "grpc.handler", 0, 5000);
        acc.record_child_span("s2", "s1", "db.query", 100, 4000);
        let mut cost = GoCostAccessor::new();
        cost.record(CostRecord::new("go-trace-001", 200, 100, "claude-3-haiku"));
        println!("[Go] spans={} total_tokens={}", acc.span_count(), cost.total_tokens());
    }

    // Python
    {
        let mut acc = PyTraceAccessor::new("py-trace-001");
        acc.record_span("s1", "fastapi.route", 0, 3000);
        let mut cost = PyCostAccessor::new();
        cost.record(CostRecord::new("py-trace-001", 300, 150, "claude-3-sonnet"));
        println!("[Python] spans={} summary={}", acc.span_count(), cost.summarize());
    }

    // TypeScript
    {
        let mut acc = TsTraceAccessor::new("ts-trace-001");
        acc.record_span("s1", "express.handler", 0, 2000);
        let json = acc.to_json_strings();
        let mut cost = TsCostAccessor::new();
        cost.record(CostRecord::new("ts-trace-001", 150, 75, "claude-3-haiku"));
        println!("[TypeScript] spans={} json_ok={} total_tokens={}", acc.span_count(), !json.is_empty(), cost.total_tokens());
    }

    // .NET
    {
        let mut acc = DotnetTraceAccessor::new("dotnet-trace-001");
        acc.start_activity("s1", "MVC.Action", 0, 4000);
        let mut cost = DotnetCostAccessor::new();
        cost.record(CostRecord::new("dotnet-trace-001", 400, 200, "claude-3-opus"));
        println!("[.NET] traceparent_prefix=00- spans={} total_tokens={}", acc.span_count(), cost.total_tokens());
    }

    // Java
    {
        let mut acc = JavaTraceAccessor::new("java-trace-001");
        acc.start_span("s1", "Servlet.doGet", 0, 6000);
        acc.start_child_span("s2", "s1", "JDBC.query", 100, 5000);
        let mut cost = JavaCostAccessor::new();
        cost.record(CostRecord::new("java-trace-001", 500, 250, "claude-3-sonnet"));
        println!("[Java] spans={} cost_summary={}", acc.span_count(), cost.to_string());
    }

    // Rust
    {
        let mut acc = RsTraceAccessor::new("rs-trace-001");
        acc.record_span("s1", "axum.handler", 0, 1500);
        let mut cost = RsCostAccessor::new();
        cost.record(CostRecord::new("rs-trace-001", 100, 50, "claude-3-haiku"));
        let models: Vec<&str> = cost.iter_models().collect();
        println!("[Rust] root_dur={:?}ns models={:?}", acc.root_duration_ns(), models);
    }

    // Notebook render
    {
        let trace = make_trace("nb-trace-001");
        let renderer = NotebookTraceRenderer::new();
        let plain = renderer.render_plain(&trace);
        let html = renderer.render_html(&trace);
        println!("\n[Notebook] plain_len={} html_has_table={}", plain.content.len(), html.content.contains("<table>"));
    }

    // Multilang eval
    {
        let go_t = make_trace("go-eval");
        let py_t = make_trace("py-eval");
        let ts_t = make_trace("ts-eval");
        let dotnet_t = make_trace("dotnet-eval");
        let java_t = make_trace("java-eval");
        let rs_t = make_trace("rs-eval");

        let criteria = EvalCriteria::new("sdk-coverage")
            .with_min_spans(2)
            .with_required_span("agent.run");

        let results = run_multilang_eval(
            &[
                ("go", &go_t),
                ("python", &py_t),
                ("typescript", &ts_t),
                ("dotnet", &dotnet_t),
                ("java", &java_t),
                ("rust", &rs_t),
            ],
            &criteria,
        );

        println!("\n[Eval] results:");
        for lr in &results {
            println!("  lang={} passed={} score={:.1}", lr.language, lr.result.passed, lr.result.score);
        }

        // Single eval
        let trace = make_trace("single-eval");
        let single_criteria = EvalCriteria::new("single").with_min_spans(1);
        let result = EvalRunner::new().evaluate(&trace, &single_criteria);
        println!("\n[Single eval] passed={} notes={}", result.passed, result.notes);
    }

    println!("\n=== done ===");
}
