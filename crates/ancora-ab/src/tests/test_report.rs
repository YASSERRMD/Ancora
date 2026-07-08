use crate::analysis::welch_t_test;
use crate::experiment::{Experiment, Metric, MetricKind, Variant};
use crate::lifecycle::LifecycleManager;
use crate::outcome::{Observation, OutcomeStore};
use crate::report::ExperimentReport;

#[test]
fn report_generated_with_all_fields() {
    let experiment = Experiment::new(
        "report-exp",
        "Report generation test",
        vec![Variant::new("control", 0.5), Variant::new("treatment", 0.5)],
        Metric::new("conversion_rate", MetricKind::Maximize),
    )
    .unwrap();

    let mut lifecycle = LifecycleManager::new("report-exp");
    lifecycle.start().unwrap();

    let mut store = OutcomeStore::new();
    for i in 0..30 {
        store.record(Observation::new(
            "report-exp",
            format!("c{i}"),
            "control",
            0.4,
        ));
        store.record(Observation::new(
            "report-exp",
            format!("t{i}"),
            "treatment",
            0.7,
        ));
    }

    let ctrl_stats = store.stats_for_variant("report-exp", "control").unwrap();
    let trt_stats = store.stats_for_variant("report-exp", "treatment").unwrap();
    let sig = welch_t_test(&ctrl_stats, &trt_stats, 0.05).unwrap();

    lifecycle.conclude(Some("treatment".to_string())).unwrap();

    let report = ExperimentReport::build(&experiment, &lifecycle, &store, Some(sig));

    assert_eq!(report.experiment_id, "report-exp");
    assert_eq!(report.metric_name, "conversion_rate");
    assert_eq!(report.winner, Some("treatment".to_string()));
    assert_eq!(report.variants.len(), 2);
    assert!(report.state_summary.contains("Concluded"));
    assert!(!report.notes.is_empty());
}

#[test]
fn report_render_contains_key_info() {
    let experiment = Experiment::new(
        "render-exp",
        "Render test",
        vec![Variant::new("ctrl", 0.5), Variant::new("trt", 0.5)],
        Metric::new("score", MetricKind::Maximize),
    )
    .unwrap();

    let mut lifecycle = LifecycleManager::new("render-exp");
    lifecycle.start().unwrap();
    lifecycle.conclude(None).unwrap();

    let store = OutcomeStore::new();
    let report = ExperimentReport::build(&experiment, &lifecycle, &store, None);
    let rendered = report.render();

    assert!(rendered.contains("render-exp"));
    assert!(rendered.contains("score"));
    assert!(rendered.contains("ctrl"));
    assert!(rendered.contains("trt"));
}

#[test]
fn report_with_no_significance_has_no_sig_note() {
    let experiment = Experiment::new(
        "nosig-exp",
        "No sig",
        vec![Variant::new("a", 1.0)],
        Metric::new("m", MetricKind::Maximize),
    )
    .unwrap();
    let mut lifecycle = LifecycleManager::new("nosig-exp");
    lifecycle.start().unwrap();
    lifecycle.conclude(None).unwrap();
    let store = OutcomeStore::new();
    let report = ExperimentReport::build(&experiment, &lifecycle, &store, None);
    assert!(report.notes.is_empty());
}
