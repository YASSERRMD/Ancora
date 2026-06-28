use crate::eval_parity::{shared_eval_dataset, run_eval, EvalRunSummary, check_eval_parity};
use crate::cost_parity::{reference_cost_record, check_cost_parity};

const ALL_LANGS: &[&str] = &["rust", "python", "typescript", "go", "java", "csharp"];

#[test]
fn test_eval_scores_equal_across_all_languages() {
    let cases = shared_eval_dataset();
    let summaries: Vec<EvalRunSummary> = ALL_LANGS
        .iter()
        .map(|&lang| {
            let results = run_eval(lang, &cases);
            EvalRunSummary::from_results(lang, &results)
        })
        .collect();

    let issues = check_eval_parity(&summaries, 0.001);
    assert!(issues.is_empty(), "eval score parity issues: {:?}", issues);
}

#[test]
fn test_mean_score_is_one_for_all_languages() {
    let cases = shared_eval_dataset();
    for &lang in ALL_LANGS {
        let results = run_eval(lang, &cases);
        let summary = EvalRunSummary::from_results(lang, &results);
        assert!(
            (summary.mean_score - 1.0).abs() < 1e-9,
            "language {:?} mean_score should be 1.0, got {}",
            lang, summary.mean_score
        );
    }
}

#[test]
fn test_cost_scores_equal_across_languages() {
    let records: Vec<_> = ALL_LANGS.iter().map(|l| reference_cost_record(*l)).collect();
    let issues = check_cost_parity(&records);
    assert!(issues.is_empty(), "cost parity issues: {:?}", issues);
}

#[test]
fn test_no_language_is_missing_from_eval() {
    let cases = shared_eval_dataset();
    let summaries: Vec<EvalRunSummary> = ALL_LANGS
        .iter()
        .map(|&lang| {
            let results = run_eval(lang, &cases);
            EvalRunSummary::from_results(lang, &results)
        })
        .collect();

    assert_eq!(
        summaries.len(),
        ALL_LANGS.len(),
        "every language must have an eval summary"
    );
}
