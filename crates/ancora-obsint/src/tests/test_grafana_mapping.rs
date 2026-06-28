use crate::grafana::{map_span_to_tempo, metric_to_loki_line, GrafanaConfig, LokiStream};
use crate::otlp::{OtlpMetricPoint, OtlpSpan};

#[test]
fn test_grafana_config_with_auth() {
    let cfg = GrafanaConfig::new("http://tempo:3200", "http://loki:3100")
        .with_auth("Bearer abc123");
    assert_eq!(cfg.auth_token.as_deref(), Some("Bearer abc123"));
}

#[test]
fn test_grafana_config_with_org_id() {
    let cfg = GrafanaConfig::new("http://tempo:3200", "http://loki:3100")
        .with_org_id("42");
    assert_eq!(cfg.org_id.as_deref(), Some("42"));
}

#[test]
fn test_map_span_to_tempo_operation_name() {
    let span = OtlpSpan::new("fetch-data", [0xaau8; 16], [0xbbu8; 8]);
    let ts = map_span_to_tempo(&span);
    assert_eq!(ts.operation_name, "fetch-data");
}

#[test]
fn test_map_span_to_tempo_start_time_us() {
    let mut span = OtlpSpan::new("op", [1u8; 16], [2u8; 8]);
    span.start_ns = 5_000_000;
    let ts = map_span_to_tempo(&span);
    assert_eq!(ts.start_time_us, 5000);
}

#[test]
fn test_map_span_to_tempo_duration_us() {
    let mut span = OtlpSpan::new("op", [1u8; 16], [2u8; 8]);
    span.start_ns = 1_000_000;
    span.end_ns = 3_000_000;
    let ts = map_span_to_tempo(&span);
    assert_eq!(ts.duration_us, 2000);
}

#[test]
fn test_loki_stream_push_entry() {
    let mut stream = LokiStream::new(vec![("app".to_string(), "ancora".to_string())]);
    stream.push_entry(123_000_000, "hello loki");
    assert_eq!(stream.entries.len(), 1);
    assert_eq!(stream.entries[0].line, "hello loki");
    assert_eq!(stream.entries[0].timestamp_ns, 123_000_000);
}

#[test]
fn test_metric_to_loki_line_format() {
    let mut point = OtlpMetricPoint::new("requests_total", 42.0);
    point.labels.push(("method".to_string(), "POST".to_string()));
    point.timestamp_ns = 1_000_000_000;
    let line = metric_to_loki_line(&point);
    assert!(line.contains("requests_total"));
    assert!(line.contains("42"));
    assert!(line.contains("method=POST"));
}

#[test]
fn test_metric_to_loki_line_no_labels() {
    let point = OtlpMetricPoint::new("cpu_usage", 0.75);
    let line = metric_to_loki_line(&point);
    assert!(line.contains("cpu_usage"));
    assert!(line.contains("0.75"));
}
