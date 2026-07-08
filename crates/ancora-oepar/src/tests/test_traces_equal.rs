use crate::trace_parity::{compare_traces, reference_trace, Language};

#[test]
fn test_traces_equal_rust_vs_python() {
    let a = reference_trace(Language::Rust);
    let b = reference_trace(Language::Python);
    let result = compare_traces(&a, &b);
    assert!(result.is_equal, "traces differ: {:?}", result.differences);
}

#[test]
fn test_traces_equal_typescript_vs_go() {
    let a = reference_trace(Language::TypeScript);
    let b = reference_trace(Language::Go);
    let result = compare_traces(&a, &b);
    assert!(result.is_equal, "traces differ: {:?}", result.differences);
}

#[test]
fn test_traces_equal_java_vs_csharp() {
    let a = reference_trace(Language::Java);
    let b = reference_trace(Language::CSharp);
    let result = compare_traces(&a, &b);
    assert!(result.is_equal, "traces differ: {:?}", result.differences);
}

#[test]
fn test_all_six_traces_equal() {
    let all = Language::all();
    let traces: Vec<_> = all.iter().map(|l| reference_trace(l.clone())).collect();
    for i in 0..traces.len() {
        for j in (i + 1)..traces.len() {
            let result = compare_traces(&traces[i], &traces[j]);
            assert!(
                result.is_equal,
                "{} vs {} not equal: {:?}",
                traces[i].language.as_str(),
                traces[j].language.as_str(),
                result.differences
            );
        }
    }
}

#[test]
fn test_trace_ids_consistent() {
    let langs = Language::all();
    for lang in langs {
        let trace = reference_trace(lang);
        for span in &trace.spans {
            assert_eq!(
                span.trace_id, trace.trace_id,
                "span trace_id must match trace id"
            );
        }
    }
}
