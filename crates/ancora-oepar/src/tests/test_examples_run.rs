/// Smoke-tests that verify the example logic runs without panicking.
/// These mirror the obs_eval_example binary logic.
use crate::trace_parity::{Language, reference_trace, compare_traces};
use crate::cost_parity::{reference_cost_record, check_cost_parity};
use crate::eval_parity::{shared_eval_dataset, run_eval, EvalRunSummary, check_eval_parity};
use crate::polyglot::reference_polyglot_trace;

const LANGUAGES: &[&str] = &["rust", "python", "typescript", "go", "java", "csharp"];

#[test]
fn test_example_trace_parity_runs() {
    let reference = reference_trace(Language::Rust);
    for lang in Language::all() {
        let trace = reference_trace(lang);
        let result = compare_traces(&reference, &trace);
        assert!(result.is_equal);
    }
}

#[test]
fn test_example_cost_parity_runs() {
    let records: Vec<_> = LANGUAGES.iter().map(|l| reference_cost_record(*l)).collect();
    let issues = check_cost_parity(&records);
    assert!(issues.is_empty());
}

#[test]
fn test_example_eval_parity_runs() {
    let cases = shared_eval_dataset();
    let summaries: Vec<EvalRunSummary> = LANGUAGES
        .iter()
        .map(|&lang| {
            let results = run_eval(lang, &cases);
            EvalRunSummary::from_results(lang, &results)
        })
        .collect();
    let issues = check_eval_parity(&summaries, 0.01);
    assert!(issues.is_empty());
}

#[test]
fn test_example_polyglot_trace_runs() {
    let trace = reference_polyglot_trace();
    assert_eq!(trace.span_count(), 6);
    assert!(trace.validate_parent_links().is_empty());
}
