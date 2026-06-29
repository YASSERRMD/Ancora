//! Edge eval report tests.

use crate::report::{EdgeEvalReport, ModelEvalSummary};

fn make_summary(name: &str, cap: f64, lat: f64, mem: f64, rel: f64) -> ModelEvalSummary {
    ModelEvalSummary {
        model_name: name.to_string(),
        capability_pass_rate: cap,
        mean_latency_ms: lat,
        memory_total_mib: mem,
        power_tokens_per_joule: 0.0,
        reliability_score: rel,
        best_quant_format: None,
    }
}

#[test]
fn test_edge_score_higher_for_better_model() {
    let good = make_summary("good", 0.9, 50.0, 500.0, 0.9);
    let poor = make_summary("poor", 0.4, 5000.0, 10000.0, 0.3);
    assert!(good.edge_score() > poor.edge_score());
}

#[test]
fn test_report_best_model_selection() {
    let mut report = EdgeEvalReport::new("test");
    report.add_summary(make_summary("a", 0.9, 50.0, 500.0, 0.9));
    report.add_summary(make_summary("b", 0.4, 5000.0, 10000.0, 0.3));
    let best = report.best_model().unwrap();
    assert_eq!(best.model_name, "a");
}

#[test]
fn test_report_render_contains_header() {
    let mut report = EdgeEvalReport::new("My Report");
    report.add_summary(make_summary("phi-mini", 0.8, 100.0, 800.0, 0.85));
    let text = report.render_text();
    assert!(text.contains("My Report"));
    assert!(text.contains("phi-mini"));
}

#[test]
fn test_report_render_empty() {
    let report = EdgeEvalReport::new("Empty");
    let text = report.render_text();
    assert!(text.contains("Empty"));
    assert!(report.best_model().is_none());
}

#[test]
fn test_report_edge_score_clamped() {
    let s = make_summary("m", 1.0, 0.0, 0.0, 1.0);
    let score = s.edge_score();
    assert!(score >= 0.0 && score <= 1.0, "score={}", score);
}
