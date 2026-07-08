/// Tests: span attributes are complete and correctly typed.
use crate::genai_attrs::{self, *};
use crate::span::{Span, SpanStatus};

#[test]
fn all_genai_request_attrs_present() {
    let mut s = Span::root("llm-call", 0);
    genai_attrs::set_request_attrs(
        &mut s,
        provider::ANTHROPIC,
        "claude-3-opus",
        Some(2048),
        Some(0.5),
    );
    assert_eq!(get_str(&s, GEN_AI_SYSTEM), Some(provider::ANTHROPIC));
    assert_eq!(get_str(&s, GEN_AI_REQUEST_MODEL), Some("claude-3-opus"));
    assert_eq!(get_int(&s, GEN_AI_REQUEST_MAX_TOKENS), Some(2048));
    let temp = get_float(&s, GEN_AI_REQUEST_TEMPERATURE).unwrap();
    assert!((temp - 0.5).abs() < 1e-9);
}

#[test]
fn response_attrs_correct() {
    let mut s = Span::root("llm-call", 0);
    genai_attrs::set_response_attrs(&mut s, "claude-3-opus-20240229", 1000, 500);
    assert_eq!(
        get_str(&s, GEN_AI_RESPONSE_MODEL),
        Some("claude-3-opus-20240229")
    );
    assert_eq!(get_int(&s, GEN_AI_USAGE_INPUT_TOKENS), Some(1000));
    assert_eq!(get_int(&s, GEN_AI_USAGE_OUTPUT_TOKENS), Some(500));
}

#[test]
fn run_attrs_all_set() {
    let mut s = Span::root("run", 0);
    genai_attrs::set_run_attrs(&mut s, "tenant-42", "run-999", "agent-7");
    assert_eq!(get_str(&s, ANCORA_TENANT_ID), Some("tenant-42"));
    assert_eq!(get_str(&s, ANCORA_RUN_ID), Some("run-999"));
    assert_eq!(get_str(&s, ANCORA_AGENT_ID), Some("agent-7"));
}

#[test]
fn error_attrs_set() {
    let mut s = Span::root("run", 0);
    genai_attrs::set_error_attr(&mut s, "RateLimitError", 3);
    assert_eq!(get_str(&s, ANCORA_ERROR_KIND), Some("RateLimitError"));
    assert_eq!(get_int(&s, ANCORA_RETRY_COUNT), Some(3));
}

#[test]
fn genai_keys_returns_only_genai() {
    let mut s = Span::root("run", 0);
    genai_attrs::set_request_attrs(&mut s, provider::OPENAI, "gpt-4o", None, None);
    genai_attrs::set_run_attrs(&mut s, "t", "r", "a");
    let keys = genai_attrs::genai_keys(&s);
    for k in &keys {
        assert!(k.starts_with("gen_ai."), "unexpected key: {}", k);
    }
}

#[test]
fn ancora_keys_returns_only_ancora() {
    let mut s = Span::root("run", 0);
    genai_attrs::set_run_attrs(&mut s, "t", "r", "a");
    genai_attrs::set_request_attrs(&mut s, provider::ANTHROPIC, "m", None, None);
    let keys = genai_attrs::ancora_keys(&s);
    for k in &keys {
        assert!(k.starts_with("ancora."), "unexpected key: {}", k);
    }
}

#[test]
fn span_status_error_preserved() {
    let mut s = Span::root("fail", 0);
    s.finish(
        100,
        SpanStatus::Error {
            code: 500,
            message: "internal error".into(),
        },
    );
    match s.status {
        SpanStatus::Error { code, ref message } => {
            assert_eq!(code, 500);
            assert_eq!(message, "internal error");
        }
        _ => panic!("expected error status"),
    }
}
