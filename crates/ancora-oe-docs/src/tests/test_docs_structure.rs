use crate::{
    ab_testing::{Experiment, ExperimentResults},
    cont_eval::{ContEvalRegistry, EvalSchedule, ScheduledEval},
    cost_analytics::{CostAggregator, ModelPricing, TokenUsage},
    datasets_graders::{Dataset, EvalSample, ExactMatchGrader, Grader, SubstringGrader},
    dev_studio::{DevStudioConfig, PromptVariant},
    drift_mon::{DriftMonitor, ScoreSample},
    eval_library::{EvalCatalog, EvalCatalogEntry, EvalCategory},
    evals_platform::{EvalResult, EvalRunSummary, EvalSpec},
    examples_index::{build_default_index, ExamplesIndex},
    feedback_review::{FeedbackItem, FeedbackSignal, ReviewQueue},
    obs_integrations::{Backend, SinkConfig, SinkRegistry},
    overview::{ObservabilityOverview, ObservabilityPillars},
    per_lang::{recommended_tracing_package, LangSdkInfo, SdkLanguage},
    readiness::build_default_checklist,
    regression_gates::{run_gates, RegressionGate},
    safety_mon::{SafetyEvent, SafetyMonitor, Severity},
    semantic_conv::{is_known_key, Attribute, SemanticAttributes},
    telemetry_priv::{FieldPolicy, PrivacyFilter, PrivacyStrategy},
    trace_model::{Span, SpanId, SpanKind, Trace, TraceId},
    troubleshooting::{IssueCategory, KnownIssue, TroubleshootingKb},
};

#[test]
fn test_overview_fully_observable() {
    let ov = ObservabilityOverview::new("prod");
    assert!(ov.is_fully_observable());
}

#[test]
fn test_pillars_active_count() {
    let p = ObservabilityPillars::default();
    assert_eq!(p.active_count(), 3); // eval is false by default
}

#[test]
fn test_trace_span_duration() {
    let trace_id = TraceId("t1".to_string());
    let span_id = SpanId("s1".to_string());
    let mut span = Span::root(trace_id, span_id, "root");
    span.start_ns = 100;
    span.finish(200);
    assert_eq!(span.duration_ns(), Some(100));
}

#[test]
fn test_trace_spans_of_kind() {
    let mut trace = Trace::default();
    let trace_id = TraceId("t1".to_string());
    let mut s = Span::root(trace_id.clone(), SpanId("s1".to_string()), "agent");
    s.kind = SpanKind::Llm;
    trace.add_span(s);
    assert_eq!(trace.spans_of_kind(&SpanKind::Llm).len(), 1);
    assert_eq!(trace.spans_of_kind(&SpanKind::Tool).len(), 0);
}

#[test]
fn test_semantic_known_key() {
    assert!(is_known_key(SemanticAttributes::AGENT_ID));
    assert!(!is_known_key("unknown.key"));
}

#[test]
fn test_attribute_types() {
    let a = Attribute::string("k", "v");
    let b = Attribute::int("k2", 42);
    let c = Attribute::float("k3", 3.14);
    let d = Attribute::bool("k4", true);
    assert_eq!(a.key, "k");
    assert_eq!(b.key, "k2");
    assert_eq!(c.key, "k3");
    assert_eq!(d.key, "k4");
}

#[test]
fn test_cost_aggregator() {
    let pricing = ModelPricing::new("test-model", 1.0, 2.0);
    let usage = TokenUsage::new(1000, 500);
    let cost = pricing.cost_usd(&usage);
    assert!((cost - 2.0).abs() < 1e-9); // 1.0 + 1.0

    let mut agg = CostAggregator::default();
    agg.record(cost, usage.total());
    assert_eq!(agg.request_count, 1);
    assert!((agg.average_cost_per_request() - 2.0).abs() < 1e-9);
}

#[test]
fn test_drift_monitor() {
    let samples: Vec<ScoreSample> = (0..10).map(|i| ScoreSample::new(i, 0.9)).collect();
    let mut monitor = DriftMonitor::new(3.0);
    monitor.calibrate(&samples).unwrap();
    // Same score: no drift
    assert!(!monitor.is_drifted(0.9).unwrap());
}

#[test]
fn test_safety_monitor_critical() {
    let mut mon = SafetyMonitor::default();
    mon.record(SafetyEvent::new(
        Severity::Critical,
        "pol-001",
        "test violation",
        "agent-1",
    ));
    assert!(mon.has_critical());
    assert_eq!(mon.critical_count(), 1);
}

#[test]
fn test_privacy_filter_redact() {
    let policies = vec![FieldPolicy::redact("email"), FieldPolicy::drop("ssn")];
    let filter = PrivacyFilter::new(policies, PrivacyStrategy::Allow);
    let attrs = vec![
        ("name".to_string(), "Alice".to_string()),
        ("email".to_string(), "alice@example.com".to_string()),
        ("ssn".to_string(), "123-45-6789".to_string()),
    ];
    let result = filter.apply(attrs);
    assert_eq!(result.len(), 2); // ssn dropped
    let email = result.iter().find(|(k, _)| k == "email").unwrap();
    assert_eq!(email.1, "[REDACTED]");
}

#[test]
fn test_eval_run_summary() {
    let mut summary = EvalRunSummary::default();
    summary.add(EvalResult::new("e1", "s1", 0.9, 0.8));
    summary.add(EvalResult::new("e1", "s2", 0.7, 0.8));
    assert!((summary.pass_rate() - 0.5).abs() < 1e-9);
    assert!((summary.mean_score() - 0.8).abs() < 1e-9);
}

#[test]
fn test_graders() {
    let exact = ExactMatchGrader;
    assert_eq!(exact.grade("hello", "hello"), 1.0);
    assert_eq!(exact.grade("hello", "world"), 0.0);

    let sub = SubstringGrader;
    assert_eq!(sub.grade("hello world", "world"), 1.0);
    assert_eq!(sub.grade("hello", "world"), 0.0);
}

#[test]
fn test_dataset() {
    let mut ds = Dataset::new("test_ds");
    ds.add(EvalSample::new("s1", "prompt1").with_expected("answer1"));
    assert_eq!(ds.len(), 1);
    assert!(!ds.is_empty());
}

#[test]
fn test_regression_gate_pass() {
    let gate = RegressionGate::new("gate1", 0.8, 0.7);
    let mut summary = EvalRunSummary::default();
    for _ in 0..10 {
        summary.add(EvalResult::new("e1", "s", 0.9, 0.8));
    }
    assert!(run_gates(&[gate], &summary).is_none());
}

#[test]
fn test_regression_gate_fail() {
    let gate = RegressionGate::new("gate1", 0.95, 0.7);
    let mut summary = EvalRunSummary::default();
    summary.add(EvalResult::new("e1", "s1", 0.9, 0.8));
    summary.add(EvalResult::new("e1", "s2", 0.6, 0.8));
    assert!(run_gates(&[gate], &summary).is_some());
}

#[test]
fn test_ab_experiment_weights() {
    let exp = Experiment::two_way("exp1", "Test Experiment");
    assert!(exp.validate_weights());
}

#[test]
fn test_experiment_results_winner() {
    let mut results = ExperimentResults::default();
    results.record("control", 0.7);
    results.record("treatment", 0.9);
    assert_eq!(results.winner(), Some("treatment"));
}

#[test]
fn test_review_queue_positive_ratio() {
    let mut q = ReviewQueue::default();
    q.enqueue(FeedbackItem::new("o1", FeedbackSignal::Positive, "r1"));
    q.enqueue(FeedbackItem::new("o2", FeedbackSignal::Negative, "r2"));
    assert!((q.positive_ratio() - 0.5).abs() < 1e-9);
}

#[test]
fn test_cont_eval_registry() {
    let mut reg = ContEvalRegistry::default();
    reg.register(ScheduledEval::new("e1", EvalSchedule::OnDeploy));
    reg.register(ScheduledEval::new("e2", EvalSchedule::EveryMinutes(60)));
    assert_eq!(reg.total_count(), 2);
    assert_eq!(reg.on_deploy_jobs().len(), 1);
}

#[test]
fn test_dev_studio_config_defaults() {
    let cfg = DevStudioConfig::default();
    assert_eq!(cfg.port, 7878);
    assert!(cfg.enable_trace_replay);
}

#[test]
fn test_prompt_variant_variables() {
    let pv = PromptVariant::new("v1", "Hello {name}, your score is {score}.");
    assert_eq!(pv.variables, vec!["name", "score"]);
}

#[test]
fn test_sink_registry() {
    let mut reg = SinkRegistry::default();
    reg.register(SinkConfig::console("console-sink"));
    assert_eq!(reg.enabled_sinks().len(), 1);
    assert!(reg.find_by_name("console-sink").is_some());
}

#[test]
fn test_per_lang_guidance() {
    let info = LangSdkInfo::for_language(SdkLanguage::Rust);
    assert_eq!(
        info.tracing_package,
        recommended_tracing_package(&SdkLanguage::Rust)
    );
}

#[test]
fn test_eval_catalog() {
    let mut cat = EvalCatalog::default();
    cat.add(EvalCatalogEntry::new(
        "e1",
        "Factuality Bench",
        EvalCategory::Factuality,
        100,
        "exact_match",
    ));
    cat.add(EvalCatalogEntry::new(
        "e2",
        "Code Bench",
        EvalCategory::CodeGen,
        50,
        "exact_match",
    ));
    assert_eq!(cat.total(), 2);
    assert_eq!(cat.by_category(&EvalCategory::Factuality).len(), 1);
}

#[test]
fn test_troubleshooting_kb() {
    let mut kb = TroubleshootingKb::default();
    kb.add(
        KnownIssue::new(
            "t001",
            IssueCategory::Tracing,
            "No spans exported",
            "Check OTLP endpoint config.",
        )
        .with_step("Verify OTEL_EXPORTER_OTLP_ENDPOINT is set"),
    );
    assert_eq!(kb.total_issues(), 1);
    assert!(kb.find_by_id("t001").is_some());
}

#[test]
fn test_examples_index_default() {
    let idx = build_default_index();
    assert!(idx.total() >= 3);
    assert!(!idx.by_tag("tracing").is_empty());
}

#[test]
fn test_eval_spec_fields() {
    let spec = EvalSpec::new("e1", "Accuracy", "Test accuracy", "exact_match");
    assert_eq!(spec.id, "e1");
    assert_eq!(spec.grader_id, "exact_match");
}
