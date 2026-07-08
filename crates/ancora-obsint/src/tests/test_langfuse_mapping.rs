use crate::langfuse::{extract_trace_id, map_span_to_observation, LangfuseConfig, ObservationKind};
use crate::otlp::OtlpSpan;

#[test]
fn test_langfuse_config_base_url() {
    let cfg = LangfuseConfig::new("https://cloud.langfuse.com", "pk-test", "sk-test");
    assert_eq!(cfg.base_url(), "https://cloud.langfuse.com/api/public");
}

#[test]
fn test_langfuse_config_trailing_slash() {
    let cfg = LangfuseConfig::new("https://cloud.langfuse.com/", "pk", "sk");
    assert_eq!(cfg.base_url(), "https://cloud.langfuse.com/api/public");
}

#[test]
fn test_span_to_observation_kind() {
    let span = OtlpSpan::new("my-span", [1u8; 16], [2u8; 8]);
    let obs = map_span_to_observation(&span);
    assert_eq!(obs.kind, ObservationKind::Span);
    assert_eq!(obs.name, "my-span");
}

#[test]
fn test_span_to_observation_duration_none_when_zero() {
    let span = OtlpSpan::new("no-duration", [0u8; 16], [0u8; 8]);
    let obs = map_span_to_observation(&span);
    assert!(obs.duration_ms.is_none());
}

#[test]
fn test_span_to_observation_duration_some() {
    let mut span = OtlpSpan::new("timed", [0u8; 16], [1u8; 8]);
    span.start_ns = 1_000_000;
    span.end_ns = 2_000_000;
    let obs = map_span_to_observation(&span);
    assert_eq!(obs.duration_ms, Some(1.0));
}

#[test]
fn test_extract_trace_id_from_attribute() {
    let mut span = OtlpSpan::new("s", [0u8; 16], [0u8; 8]);
    span.attributes
        .push(("langfuse.trace_id".to_string(), "custom-id".to_string()));
    assert_eq!(extract_trace_id(&span), "custom-id");
}

#[test]
fn test_extract_trace_id_fallback() {
    let span = OtlpSpan::new("s", [0xabu8; 16], [0u8; 8]);
    let tid = extract_trace_id(&span);
    assert!(tid.starts_with("ab"));
}

#[test]
fn test_langfuse_with_project() {
    let cfg = LangfuseConfig::new("https://host", "pk", "sk").with_project("proj-1");
    assert_eq!(cfg.project_id.as_deref(), Some("proj-1"));
}
