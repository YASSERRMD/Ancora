use crate::health::{summarize_health, ExporterHealthReport, HealthChecker, HealthStatus};
use crate::selection::ExporterBackend;

#[test]
fn test_health_checker_healthy_below_threshold() {
    let checker = HealthChecker::new(vec![ExporterBackend::Otlp]);
    let report = checker.evaluate(ExporterBackend::Otlp, 100);
    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.latency_ms, Some(100));
}

#[test]
fn test_health_checker_degraded_between_thresholds() {
    let checker = HealthChecker::new(vec![ExporterBackend::Prometheus]).with_thresholds(500, 2000);
    let report = checker.evaluate(ExporterBackend::Prometheus, 750);
    assert!(report.status.is_degraded());
    assert_eq!(report.latency_ms, Some(750));
}

#[test]
fn test_health_checker_unhealthy_above_threshold() {
    let checker = HealthChecker::new(vec![ExporterBackend::Langfuse]).with_thresholds(500, 2000);
    let report = checker.evaluate(ExporterBackend::Langfuse, 3000);
    assert!(report.status.is_unhealthy());
    assert!(report.latency_ms.is_none());
}

#[test]
fn test_exporter_health_report_healthy_constructor() {
    let report = ExporterHealthReport::healthy(ExporterBackend::GrafanaTempo, 50);
    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.latency_ms, Some(50));
}

#[test]
fn test_exporter_health_report_unhealthy_constructor() {
    let report = ExporterHealthReport::unhealthy(ExporterBackend::Datadog, "connection refused");
    assert!(report.status.is_unhealthy());
    assert!(report.latency_ms.is_none());
}

#[test]
fn test_summarize_health_all_healthy() {
    let reports = vec![
        ExporterHealthReport::healthy(ExporterBackend::Otlp, 50),
        ExporterHealthReport::healthy(ExporterBackend::Prometheus, 30),
    ];
    assert_eq!(summarize_health(&reports), HealthStatus::Healthy);
}

#[test]
fn test_summarize_health_one_degraded() {
    let checker = HealthChecker::new(vec![]).with_thresholds(200, 1000);
    let reports = vec![
        ExporterHealthReport::healthy(ExporterBackend::Otlp, 50),
        checker.evaluate(ExporterBackend::Prometheus, 500),
    ];
    assert!(summarize_health(&reports).is_degraded());
}

#[test]
fn test_summarize_health_all_unhealthy() {
    let reports = vec![
        ExporterHealthReport::unhealthy(ExporterBackend::Otlp, "timeout"),
        ExporterHealthReport::unhealthy(ExporterBackend::Datadog, "refused"),
    ];
    assert!(summarize_health(&reports).is_unhealthy());
}

#[test]
fn test_summarize_health_empty_is_unhealthy() {
    assert!(summarize_health(&[]).is_unhealthy());
}

#[test]
fn test_health_status_label() {
    assert_eq!(HealthStatus::Healthy.label(), "healthy");
    assert_eq!(
        HealthStatus::Degraded {
            reason: "slow".to_string()
        }
        .label(),
        "degraded"
    );
    assert_eq!(
        HealthStatus::Unhealthy {
            reason: "down".to_string()
        }
        .label(),
        "unhealthy"
    );
}
