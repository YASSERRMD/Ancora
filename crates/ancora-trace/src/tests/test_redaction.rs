/// Tests: redaction policy applied per policy rules.
use crate::genai_attrs::{GEN_AI_COMPLETION, GEN_AI_PROMPT, GEN_AI_REQUEST_MODEL};
use crate::redact::{RedactPolicy, REDACTED_SENTINEL};
use crate::span::Span;

#[test]
fn passthrough_does_not_modify_attrs() {
    let policy = RedactPolicy::passthrough();
    let mut s = Span::root("llm", 0);
    s.set_attr_str(GEN_AI_PROMPT, "hello world");
    let out = policy.apply_to_span(&s);
    let v = out.attributes.get(GEN_AI_PROMPT).unwrap();
    assert_eq!(v.as_str(), Some("hello world"));
}

#[test]
fn redact_content_replaces_prompt_and_completion() {
    let policy = RedactPolicy::redact_content();
    let mut s = Span::root("llm", 0);
    s.set_attr_str(GEN_AI_PROMPT, "user secret query");
    s.set_attr_str(GEN_AI_COMPLETION, "model secret response");
    s.set_attr_str(GEN_AI_REQUEST_MODEL, "claude-3");
    let out = policy.apply_to_span(&s);

    let prompt = out.attributes.get(GEN_AI_PROMPT).unwrap();
    assert_eq!(prompt.as_str(), Some(REDACTED_SENTINEL));

    let completion = out.attributes.get(GEN_AI_COMPLETION).unwrap();
    assert_eq!(completion.as_str(), Some(REDACTED_SENTINEL));

    // Non-content fields must survive.
    let model = out.attributes.get(GEN_AI_REQUEST_MODEL).unwrap();
    assert_eq!(model.as_str(), Some("claude-3"));
}

#[test]
fn truncate_policy_shortens_long_prompt() {
    let policy = RedactPolicy::truncate_content(20);
    let long_prompt = "a".repeat(100);
    let mut s = Span::root("llm", 0);
    s.set_attr_str(GEN_AI_PROMPT, long_prompt.as_str());
    let out = policy.apply_to_span(&s);
    let v = out.attributes.get(GEN_AI_PROMPT).unwrap().as_str().unwrap();
    assert!(v.len() <= 23, "truncated value too long: {}", v.len());
    assert!(v.ends_with("..."));
}

#[test]
fn truncate_policy_leaves_short_prompts() {
    let policy = RedactPolicy::truncate_content(100);
    let short = "short prompt";
    let mut s = Span::root("llm", 0);
    s.set_attr_str(GEN_AI_PROMPT, short);
    let out = policy.apply_to_span(&s);
    let v = out.attributes.get(GEN_AI_PROMPT).unwrap().as_str().unwrap();
    assert_eq!(v, short);
}

#[test]
fn redact_does_not_change_non_content_integer_attrs() {
    let policy = RedactPolicy::redact_content();
    let mut s = Span::root("llm", 0);
    s.set_attr_int("gen_ai.usage.input_tokens", 1234);
    let out = policy.apply_to_span(&s);
    let v = out.attributes.get("gen_ai.usage.input_tokens").unwrap();
    assert_eq!(v.as_int(), Some(1234));
}

#[test]
fn should_redact_matches_prefix() {
    let policy = RedactPolicy::redact_content();
    assert!(policy.should_redact("gen_ai.prompt"));
    assert!(policy.should_redact("gen_ai.prompt.extra"));
    assert!(!policy.should_redact("gen_ai.usage.input_tokens"));
    assert!(!policy.should_redact("ancora.tenant.id"));
}

#[test]
fn original_span_unmodified_after_policy_apply() {
    let policy = RedactPolicy::redact_content();
    let mut s = Span::root("llm", 0);
    s.set_attr_str(GEN_AI_PROMPT, "original");
    let _out = policy.apply_to_span(&s);
    // The original must not have been mutated.
    let v = s.attributes.get(GEN_AI_PROMPT).unwrap().as_str().unwrap();
    assert_eq!(v, "original");
}
