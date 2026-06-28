use crate::dashboard::{
    validate_json, AlertSummary, DashboardState, ModelSnapshot, ProviderSnapshot,
};

fn make_state() -> DashboardState {
    let models = vec![
        ModelSnapshot {
            model: "gpt-4".to_string(),
            provider: "openai".to_string(),
            mean_score: 0.87,
            latest_score: 0.9,
            observation_count: 50,
            trend_slope: Some(-0.002),
        },
        ModelSnapshot {
            model: "claude-3".to_string(),
            provider: "anthropic".to_string(),
            mean_score: 0.92,
            latest_score: 0.95,
            observation_count: 40,
            trend_slope: None,
        },
    ];
    let providers = vec![ProviderSnapshot {
        provider: "openai".to_string(),
        mean_score: 0.87,
        observation_count: 50,
        model_count: 1,
    }];
    let alert_summary = AlertSummary {
        total: 2,
        critical: 0,
        warning: 2,
        info: 0,
    };
    DashboardState::new(1_700_000_000, models, providers, alert_summary)
}

#[test]
fn test_dashboard_json_is_valid() {
    let state = make_state();
    let json = state.to_json();
    validate_json(&json).expect("dashboard JSON should be valid");
}

#[test]
fn test_dashboard_json_contains_model_name() {
    let state = make_state();
    let json = state.to_json();
    assert!(json.contains("gpt-4"), "JSON should contain model name");
    assert!(json.contains("claude-3"), "JSON should contain claude-3");
}

#[test]
fn test_dashboard_json_contains_provider() {
    let state = make_state();
    let json = state.to_json();
    assert!(json.contains("openai"));
    assert!(json.contains("anthropic"));
}

#[test]
fn test_dashboard_json_contains_alert_summary() {
    let state = make_state();
    let json = state.to_json();
    assert!(json.contains("alert_summary"));
    assert!(json.contains("\"warning\":2"));
}

#[test]
fn test_dashboard_json_null_trend_slope() {
    let state = make_state();
    let json = state.to_json();
    // claude-3 has None trend_slope - should appear as null.
    assert!(json.contains("null"), "None trend_slope should serialize as null");
}

#[test]
fn test_validate_json_accepts_valid_object() {
    let valid = r#"{"key":"value","nested":{"a":1},"arr":[1,2,3]}"#;
    assert!(validate_json(valid).is_ok());
}

#[test]
fn test_validate_json_rejects_non_object() {
    assert!(validate_json("[1,2,3]").is_err());
    assert!(validate_json("\"string\"").is_err());
}

#[test]
fn test_validate_json_rejects_unbalanced_braces() {
    assert!(validate_json("{\"a\":1").is_err());
}

#[test]
fn test_alert_summary_from_alerts_counts_correctly() {
    use crate::alerting::{AlertConfig, AlertEngine};
    use std::time::SystemTime;

    let config = AlertConfig::new(0.6, 0.4, 0.2, 0.05);
    let mut engine = AlertEngine::new(config);
    engine.evaluate_score("m", "p", 0.35, None, SystemTime::now()); // critical
    engine.evaluate_score("m", "p", 0.55, None, SystemTime::now()); // warning
    let alerts = engine.drain_alerts();
    let summary = AlertSummary::from_alerts(&alerts);
    assert_eq!(summary.total, 2);
    assert_eq!(summary.critical, 1);
    assert_eq!(summary.warning, 1);
    assert_eq!(summary.info, 0);
}
