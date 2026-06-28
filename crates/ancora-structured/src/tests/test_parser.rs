use crate::parser::{extract_number_field, extract_string_field, parse_response};
use crate::schema::{FieldSchema, JsonType, OutputSchema};

fn schema() -> OutputSchema {
    OutputSchema::new("t")
        .add_field(FieldSchema::new("title", JsonType::String, true))
        .add_field(FieldSchema::new("score", JsonType::Number, false))
}

#[test]
fn parse_valid_response() {
    let s = schema();
    let v = parse_response(r#"{"title": "hello", "score": 9.0}"#, &s).unwrap();
    assert_eq!(extract_string_field(&v, "title"), Some("hello"));
    assert_eq!(extract_number_field(&v, "score"), Some(9.0));
}

#[test]
fn parse_invalid_response_errors() {
    let s = schema();
    let result = parse_response("bad input", &s);
    assert!(result.is_err());
}
