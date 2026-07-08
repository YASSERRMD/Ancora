//! obs_eval_example: demonstrates obs and eval parity checks for all six languages.
//!
//! This example runs fully offline and requires no network calls.
//! Run with: cargo run --example obs_eval_example -p ancora-oepar

use ancora_oepar::cost_parity::{check_cost_parity, reference_cost_record};
use ancora_oepar::eval_parity::{check_eval_parity, run_eval, shared_eval_dataset, EvalRunSummary};
use ancora_oepar::polyglot::reference_polyglot_trace;
use ancora_oepar::trace_parity::{compare_traces, reference_trace, Language};

fn main() {
    println!("=== Ancora Observability and Eval Parity Example ===\n");

    // --- Trace parity ---
    println!("--- Trace Parity ---");
    let reference = reference_trace(Language::Rust);
    let languages = Language::all();
    for lang in &languages {
        let trace = reference_trace(lang.clone());
        let result = compare_traces(&reference, &trace);
        println!(
            "  rust vs {}: {}",
            lang.as_str(),
            if result.is_equal { "EQUAL" } else { "DIFFERS" }
        );
    }

    // --- Cost parity ---
    println!("\n--- Cost Parity ---");
    let lang_names = &["rust", "python", "typescript", "go", "java", "csharp"];
    let cost_records: Vec<_> = lang_names
        .iter()
        .map(|l| reference_cost_record(*l))
        .collect();
    let cost_issues = check_cost_parity(&cost_records);
    if cost_issues.is_empty() {
        println!("  All languages: EQUAL cost attributes");
    } else {
        for issue in &cost_issues {
            println!("  ISSUE: {}", issue);
        }
    }

    // --- Eval parity ---
    println!("\n--- Eval Parity ---");
    let cases = shared_eval_dataset();
    let summaries: Vec<EvalRunSummary> = lang_names
        .iter()
        .map(|&lang| {
            let results = run_eval(lang, &cases);
            EvalRunSummary::from_results(lang, &results)
        })
        .collect();
    let eval_issues = check_eval_parity(&summaries, 0.01);
    if eval_issues.is_empty() {
        println!("  All languages: EQUAL eval scores");
    } else {
        for issue in &eval_issues {
            println!("  ISSUE: {}", issue);
        }
    }
    for s in &summaries {
        println!(
            "  {} - pass_rate={:.2} mean_score={:.4}",
            s.language,
            s.pass_rate(),
            s.mean_score
        );
    }

    // --- Polyglot trace ---
    println!("\n--- Polyglot Trace ---");
    let poly = reference_polyglot_trace();
    println!("  trace_id:    {}", poly.trace_id);
    println!("  span_count:  {}", poly.span_count());
    println!("  languages:   {}", poly.contributing_languages().len());
    let link_errors = poly.validate_parent_links();
    println!(
        "  parent links: {}",
        if link_errors.is_empty() {
            "OK"
        } else {
            "ERRORS"
        }
    );

    println!("\n=== Done ===");
}
