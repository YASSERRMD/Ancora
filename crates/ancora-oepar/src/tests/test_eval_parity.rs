use crate::eval_parity::{check_eval_parity, run_eval, shared_eval_dataset, EvalRunSummary};

#[test]
fn test_shared_dataset_has_three_cases() {
    let cases = shared_eval_dataset();
    assert_eq!(cases.len(), 3, "expected 3 shared eval cases");
}

#[test]
fn test_eval_run_returns_result_per_case() {
    let cases = shared_eval_dataset();
    let results = run_eval("rust", &cases);
    assert_eq!(results.len(), cases.len());
}

#[test]
fn test_all_cases_pass_for_each_language() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let cases = shared_eval_dataset();
    for &lang in langs {
        let results = run_eval(lang, &cases);
        for r in &results {
            assert!(
                r.passed,
                "case {:?} failed for language {:?}",
                r.case_id, lang
            );
        }
    }
}

#[test]
fn test_eval_parity_across_languages() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let cases = shared_eval_dataset();
    let summaries: Vec<_> = langs
        .iter()
        .map(|&lang| {
            let results = run_eval(lang, &cases);
            EvalRunSummary::from_results(lang, &results)
        })
        .collect();
    let issues = check_eval_parity(&summaries, 0.01);
    assert!(issues.is_empty(), "eval parity issues: {:?}", issues);
}

#[test]
fn test_eval_summary_pass_rate() {
    let cases = shared_eval_dataset();
    let results = run_eval("go", &cases);
    let summary = EvalRunSummary::from_results("go", &results);
    assert_eq!(summary.pass_rate(), 1.0, "all cases should pass");
}
