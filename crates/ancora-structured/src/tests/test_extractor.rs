use crate::extractor::JsonExtractor;

#[test]
fn direct_json_parse() {
    let text = r#"{"title": "hello", "score": 5}"#;
    let v = JsonExtractor::extract(text).unwrap();
    assert_eq!(v["title"], "hello");
}

#[test]
fn extracts_json_from_prose() {
    let text = "Here is the result: {\"title\": \"hello\"} - done.";
    let v = JsonExtractor::extract(text).unwrap();
    assert_eq!(v["title"], "hello");
}

#[test]
fn fails_on_no_json() {
    let text = "No JSON here at all.";
    assert!(JsonExtractor::extract(text).is_err());
}

#[test]
fn handles_nested_object() {
    let text = r#"{"outer": {"inner": 42}}"#;
    let v = JsonExtractor::extract(text).unwrap();
    assert_eq!(v["outer"]["inner"], 42);
}
