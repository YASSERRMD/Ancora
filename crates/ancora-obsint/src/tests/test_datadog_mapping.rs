use crate::datadog::{map_metric_to_datadog, map_span_to_datadog, DatadogConfig};
use crate::otlp::{OtlpMetricPoint, OtlpSpan};

#[test]
fn test_datadog_config_endpoints() {
    let cfg = DatadogConfig::new("key-abc", "my-service", "production");
    assert!(cfg.trace_endpoint().contains("datadoghq.com"));
    assert!(cfg.metrics_endpoint().contains("datadoghq.com"));
}

#[test]
fn test_datadog_config_custom_site() {
    let cfg = DatadogConfig::new("key", "svc", "staging").with_site("datadoghq.eu");
    assert!(cfg.trace_endpoint().contains("datadoghq.eu"));
}

#[test]
fn test_map_span_service_name() {
    let span = OtlpSpan::new("query", [1u8; 16], [2u8; 8]);
    let cfg = DatadogConfig::new("key", "ancora-svc", "prod");
    let ds = map_span_to_datadog(&span, &cfg);
    assert_eq!(ds.service, "ancora-svc");
}

#[test]
fn test_map_span_resource_from_attribute() {
    let mut span = OtlpSpan::new("http-call", [1u8; 16], [2u8; 8]);
    span.attributes.push(("http.route".to_string(), "/api/v1/health".to_string()));
    let cfg = DatadogConfig::new("key", "svc", "env");
    let ds = map_span_to_datadog(&span, &cfg);
    assert_eq!(ds.resource, "/api/v1/health");
}

#[test]
fn test_map_span_error_flag_on_status_code_2() {
    let mut span = OtlpSpan::new("err", [0u8; 16], [3u8; 8]);
    span.status_code = 2;
    let cfg = DatadogConfig::new("key", "svc", "env");
    let ds = map_span_to_datadog(&span, &cfg);
    assert_eq!(ds.error, 1);
}

#[test]
fn test_map_span_no_error_flag_on_ok() {
    let span = OtlpSpan::new("ok", [0u8; 16], [4u8; 8]);
    let cfg = DatadogConfig::new("key", "svc", "env");
    let ds = map_span_to_datadog(&span, &cfg);
    assert_eq!(ds.error, 0);
}

#[test]
fn test_map_metric_includes_env_and_service_tags() {
    let point = OtlpMetricPoint::new("ancora.requests", 10.0);
    let cfg = DatadogConfig::new("key", "ancora", "staging");
    let dm = map_metric_to_datadog(&point, &cfg);
    assert!(dm.tags.iter().any(|t| t == "env:staging"));
    assert!(dm.tags.iter().any(|t| t == "service:ancora"));
}

#[test]
fn test_map_metric_type_is_gauge() {
    let point = OtlpMetricPoint::new("mem_bytes", 1024.0);
    let cfg = DatadogConfig::new("key", "svc", "prod");
    let dm = map_metric_to_datadog(&point, &cfg);
    assert_eq!(dm.metric_type, "gauge");
}

#[test]
fn test_map_metric_label_tags_included() {
    let mut point = OtlpMetricPoint::new("latency", 200.0);
    point.labels.push(("region".to_string(), "us-west-2".to_string()));
    let cfg = DatadogConfig::new("key", "svc", "prod");
    let dm = map_metric_to_datadog(&point, &cfg);
    assert!(dm.tags.iter().any(|t| t == "region:us-west-2"));
}
