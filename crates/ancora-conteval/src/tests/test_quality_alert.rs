use crate::alerting::{AlertConfig, AlertEngine, AlertReason, AlertSeverity};
use std::time::SystemTime;

fn make_config() -> AlertConfig {
    AlertConfig::new(
        0.6,  // warning_threshold
        0.4,  // critical_threshold
        0.2,  // sudden_drop_fraction (20%)
        0.05, // degrading_slope
    )
}

#[test]
fn test_no_alert_for_high_score() {
    let mut engine = AlertEngine::new(make_config());
    engine.evaluate_score("gpt-4", "openai", 0.95, None, SystemTime::now());
    assert_eq!(engine.alert_count(), 0);
}

#[test]
fn test_warning_alert_for_score_at_warning_threshold() {
    let mut engine = AlertEngine::new(make_config());
    engine.evaluate_score("gpt-4", "openai", 0.55, None, SystemTime::now());
    assert_eq!(engine.alert_count(), 1);
    assert_eq!(engine.alerts()[0].severity, AlertSeverity::Warning);
}

#[test]
fn test_critical_alert_for_score_at_critical_threshold() {
    let mut engine = AlertEngine::new(make_config());
    engine.evaluate_score("m", "p", 0.35, None, SystemTime::now());
    assert_eq!(engine.alert_count(), 1);
    assert_eq!(engine.alerts()[0].severity, AlertSeverity::Critical);
}

#[test]
fn test_sudden_drop_raises_alert() {
    let mut engine = AlertEngine::new(make_config());
    // Previous 0.9, current 0.7 -> drop = (0.9 - 0.7) / 0.9 = 22% > 20%
    engine.evaluate_score("m", "p", 0.7, Some(0.9), SystemTime::now());
    let alerts = engine.alerts();
    let has_drop = alerts
        .iter()
        .any(|a| matches!(a.reason, AlertReason::SuddenDrop { .. }));
    assert!(has_drop);
}

#[test]
fn test_no_sudden_drop_for_small_change() {
    let mut engine = AlertEngine::new(make_config());
    // Drop = (0.9 - 0.88) / 0.9 = 2.2% < 20%
    engine.evaluate_score("m", "p", 0.88, Some(0.9), SystemTime::now());
    let alerts = engine.alerts();
    let has_drop = alerts
        .iter()
        .any(|a| matches!(a.reason, AlertReason::SuddenDrop { .. }));
    assert!(!has_drop);
}

#[test]
fn test_trend_alert_raised_for_degrading_slope() {
    let mut engine = AlertEngine::new(make_config());
    engine.raise_trend_alert("m", "p", -0.1, SystemTime::now());
    assert_eq!(engine.alert_count(), 1);
    let alert = &engine.alerts()[0];
    assert!(matches!(alert.reason, AlertReason::DegradingTrend { .. }));
}

#[test]
fn test_trend_alert_not_raised_for_mild_slope() {
    let mut engine = AlertEngine::new(make_config());
    engine.raise_trend_alert("m", "p", -0.01, SystemTime::now());
    assert_eq!(engine.alert_count(), 0);
}

#[test]
fn test_drain_alerts_clears_list() {
    let mut engine = AlertEngine::new(make_config());
    engine.evaluate_score("m", "p", 0.3, None, SystemTime::now());
    let drained = engine.drain_alerts();
    assert!(!drained.is_empty());
    assert_eq!(engine.alert_count(), 0);
}
