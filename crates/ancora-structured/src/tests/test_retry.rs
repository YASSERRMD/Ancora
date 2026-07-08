use crate::retry::{RetryConfig, StructuredRetry};
use crate::schema::{FieldSchema, JsonType, OutputSchema};

fn schema() -> OutputSchema {
    OutputSchema::new("task").add_field(FieldSchema::new("title", JsonType::String, true))
}

#[test]
fn succeeds_on_first_attempt() {
    let s = schema();
    let retry = StructuredRetry::new(RetryConfig::new(3));
    let result = retry.run(&s, |_, _| r#"{"title": "done"}"#.to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap()["title"], "done");
}

#[test]
fn retries_on_bad_response_then_succeeds() {
    let s = schema();
    let retry = StructuredRetry::new(RetryConfig::new(3));
    let mut count = 0u32;
    let result = retry.run(&s, |_, _| {
        count += 1;
        if count < 2 {
            "not json".to_string()
        } else {
            r#"{"title": "ok"}"#.to_string()
        }
    });
    assert!(result.is_ok());
}

#[test]
fn retry_limit_exceeded_errors() {
    let s = schema();
    let retry = StructuredRetry::new(RetryConfig::new(2));
    let result = retry.run(&s, |_, _| "bad response".to_string());
    assert!(result.is_err());
}
