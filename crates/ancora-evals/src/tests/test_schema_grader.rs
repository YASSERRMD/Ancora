use crate::schema_grader::{SchemaGrader, SchemaRule};

#[test]
fn test_schema_is_object_passes() {
    let g = SchemaGrader::new().with_rule(SchemaRule::IsObject);
    let result = g.validate(r#"{"key": "value"}"#);
    assert!(result.passed);
    assert!(result.violations.is_empty());
}

#[test]
fn test_schema_is_object_fails_for_array() {
    let g = SchemaGrader::new().with_rule(SchemaRule::IsObject);
    let result = g.validate("[1, 2, 3]");
    assert!(!result.passed);
    assert!(!result.violations.is_empty());
}

#[test]
fn test_schema_has_key_passes() {
    let g = SchemaGrader::new().with_rule(SchemaRule::HasKey("name".to_string()));
    let result = g.validate(r#"{"name": "Alice", "age": 30}"#);
    assert!(result.passed);
}

#[test]
fn test_schema_has_key_fails_when_missing() {
    let g = SchemaGrader::new().with_rule(SchemaRule::HasKey("email".to_string()));
    let result = g.validate(r#"{"name": "Alice"}"#);
    assert!(!result.passed);
    assert!(result.violations[0].contains("email"));
}

#[test]
fn test_schema_max_length() {
    let g = SchemaGrader::new().with_rule(SchemaRule::MaxLength(5));
    assert!(g.validate("abc").passed);
    assert!(!g.validate("abcdefg").passed);
}

#[test]
fn test_schema_non_empty() {
    let g = SchemaGrader::new().with_rule(SchemaRule::NonEmpty);
    assert!(!g.validate("").passed);
    assert!(g.validate("content").passed);
}

#[test]
fn test_schema_multiple_rules_all_must_pass() {
    let g = SchemaGrader::new()
        .with_rule(SchemaRule::IsObject)
        .with_rule(SchemaRule::HasKey("id".to_string()));
    let result = g.validate(r#"{"id": 1}"#);
    assert!(result.passed);
    let result2 = g.validate(r#"{"name": "x"}"#);
    assert!(!result2.passed);
}
