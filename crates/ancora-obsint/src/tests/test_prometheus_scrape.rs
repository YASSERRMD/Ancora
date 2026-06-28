use crate::otlp::OtlpMetricPoint;
use crate::prometheus::{
    points_to_prometheus, render_scrape, validate_metric_name, MetricType, PrometheusMetric,
};

#[test]
fn test_metric_render_no_labels() {
    let mut m = PrometheusMetric::new("my_counter", "A counter", MetricType::Counter);
    m.add_sample(vec![], 42.0);
    let rendered = m.render();
    assert!(rendered.contains("# HELP my_counter A counter"));
    assert!(rendered.contains("# TYPE my_counter counter"));
    assert!(rendered.contains("my_counter 42"));
}

#[test]
fn test_metric_render_with_labels() {
    let mut m = PrometheusMetric::new("http_requests", "HTTP requests", MetricType::Gauge);
    m.add_sample(
        vec![
            ("method".to_string(), "GET".to_string()),
            ("status".to_string(), "200".to_string()),
        ],
        100.0,
    );
    let rendered = m.render();
    assert!(rendered.contains("method=\"GET\""));
    assert!(rendered.contains("status=\"200\""));
    assert!(rendered.contains("100"));
}

#[test]
fn test_points_to_prometheus_groups_by_name() {
    let points = vec![
        OtlpMetricPoint::new("cpu_usage", 0.8),
        OtlpMetricPoint::new("cpu_usage", 0.9),
        OtlpMetricPoint::new("mem_bytes", 1024.0),
    ];
    let metrics = points_to_prometheus(&points);
    assert_eq!(metrics.len(), 2);
}

#[test]
fn test_render_scrape_non_empty() {
    let mut m = PrometheusMetric::new("test_metric", "help", MetricType::Gauge);
    m.add_sample(vec![], 1.0);
    let scrape = render_scrape(&[m]);
    assert!(!scrape.is_empty());
    assert!(scrape.contains("test_metric"));
}

#[test]
fn test_validate_metric_name_valid() {
    assert!(validate_metric_name("http_requests_total").is_ok());
    assert!(validate_metric_name("_internal_metric").is_ok());
    assert!(validate_metric_name("my:metric").is_ok());
}

#[test]
fn test_validate_metric_name_invalid_start() {
    assert!(validate_metric_name("0bad_metric").is_err());
    assert!(validate_metric_name("-bad").is_err());
}

#[test]
fn test_validate_metric_name_empty() {
    assert!(validate_metric_name("").is_err());
}

#[test]
fn test_validate_metric_name_invalid_char() {
    assert!(validate_metric_name("my-metric").is_err());
    assert!(validate_metric_name("my.metric").is_err());
}

#[test]
fn test_metric_type_as_str() {
    assert_eq!(MetricType::Counter.as_str(), "counter");
    assert_eq!(MetricType::Gauge.as_str(), "gauge");
    assert_eq!(MetricType::Histogram.as_str(), "histogram");
}
