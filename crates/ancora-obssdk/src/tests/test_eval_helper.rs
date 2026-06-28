use crate::context::{Span, Trace};
use crate::eval_helpers::{EvalCriteria, EvalRunner, run_multilang_eval};

fn make_trace(id: &str, spans: &[(&str, &str, u64, u64)]) -> Trace {
    let mut t = Trace::new(id);
    for (sid, name, start, end) in spans {
        t.add_span(Span::new(*sid, *name, *start).finish(*end));
    }
    t
}

#[test]
fn eval_helper_passes_criteria() {
    let trace = make_trace("e-t1", &[
        ("s1", "agent.run", 0, 500),
        ("s2", "tool.call", 50, 400),
    ]);
    let criteria = EvalCriteria::new("basic")
        .with_min_spans(2)
        .with_required_span("agent.run");
    let runner = EvalRunner::new();
    let result = runner.evaluate(&trace, &criteria);
    assert!(result.passed);
    assert_eq!(result.score, 1.0);
}

#[test]
fn eval_helper_fails_min_spans() {
    let trace = make_trace("e-t2", &[("s1", "only.one", 0, 100)]);
    let criteria = EvalCriteria::new("needs-two").with_min_spans(2);
    let runner = EvalRunner::new();
    let result = runner.evaluate(&trace, &criteria);
    assert!(!result.passed);
    assert_eq!(result.score, 0.0);
}

#[test]
fn eval_helper_from_each_language() {
    let go_trace = make_trace("go-e1", &[("s1", "grpc.handler", 0, 100)]);
    let py_trace = make_trace("py-e1", &[("s1", "fastapi.route", 0, 200)]);
    let ts_trace = make_trace("ts-e1", &[("s1", "express.middleware", 0, 150)]);
    let dotnet_trace = make_trace("dotnet-e1", &[("s1", "asp.action", 0, 180)]);
    let java_trace = make_trace("java-e1", &[("s1", "spring.controller", 0, 250)]);
    let rs_trace = make_trace("rs-e1", &[("s1", "axum.handler", 0, 90)]);

    let criteria = EvalCriteria::new("min-one").with_min_spans(1);
    let results = run_multilang_eval(
        &[
            ("go", &go_trace),
            ("python", &py_trace),
            ("typescript", &ts_trace),
            ("dotnet", &dotnet_trace),
            ("java", &java_trace),
            ("rust", &rs_trace),
        ],
        &criteria,
    );

    assert_eq!(results.len(), 6);
    for lr in &results {
        assert!(lr.result.passed, "lang {} failed eval", lr.language);
    }
}
