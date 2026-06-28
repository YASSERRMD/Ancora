use crate::otlp::OtlpSpan;
use crate::phoenix::{map_span_to_phoenix, validate_config, PhoenixConfig, SpanKind};

#[test]
fn test_span_kind_from_str() {
    assert_eq!(SpanKind::from_str("llm"), SpanKind::Llm);
    assert_eq!(SpanKind::from_str("LLM"), SpanKind::Llm);
    assert_eq!(SpanKind::from_str("retriever"), SpanKind::Retriever);
    assert_eq!(SpanKind::from_str("chain"), SpanKind::Chain);
    assert_eq!(SpanKind::from_str("unknown-xyz"), SpanKind::Unknown);
}

#[test]
fn test_openinference_label() {
    assert_eq!(SpanKind::Llm.openinference_label(), "LLM");
    assert_eq!(SpanKind::Chain.openinference_label(), "CHAIN");
    assert_eq!(SpanKind::Unknown.openinference_label(), "UNKNOWN");
}

#[test]
fn test_map_span_to_phoenix_unknown_kind() {
    let span = OtlpSpan::new("my-span", [1u8; 16], [2u8; 8]);
    let ps = map_span_to_phoenix(&span);
    assert_eq!(ps.kind, SpanKind::Unknown);
    assert_eq!(ps.name, "my-span");
}

#[test]
fn test_map_span_to_phoenix_llm_kind() {
    let mut span = OtlpSpan::new("llm-call", [3u8; 16], [4u8; 8]);
    span.attributes.push(("openinference.span.kind".to_string(), "LLM".to_string()));
    let ps = map_span_to_phoenix(&span);
    assert_eq!(ps.kind, SpanKind::Llm);
}

#[test]
fn test_map_span_status_message_on_error() {
    let mut span = OtlpSpan::new("err-span", [0u8; 16], [5u8; 8]);
    span.status_code = 2;
    let ps = map_span_to_phoenix(&span);
    assert!(ps.status_message.is_some());
}

#[test]
fn test_map_span_no_status_message_on_ok() {
    let span = OtlpSpan::new("ok-span", [0u8; 16], [6u8; 8]);
    let ps = map_span_to_phoenix(&span);
    assert!(ps.status_message.is_none());
}

#[test]
fn test_validate_config_ok() {
    let cfg = PhoenixConfig::new("http://localhost:6006", "my-project");
    assert!(validate_config(&cfg).is_ok());
}

#[test]
fn test_validate_config_empty_endpoint() {
    let cfg = PhoenixConfig::new("", "my-project");
    assert!(validate_config(&cfg).is_err());
}

#[test]
fn test_validate_config_empty_project() {
    let cfg = PhoenixConfig::new("http://localhost:6006", "");
    assert!(validate_config(&cfg).is_err());
}
