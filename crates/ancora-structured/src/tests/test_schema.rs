use crate::schema::{FieldSchema, JsonType, OutputSchema};

#[test]
fn schema_required_fields() {
    let s = OutputSchema::new("task")
        .add_field(FieldSchema::new("title", JsonType::String, true))
        .add_field(FieldSchema::new("score", JsonType::Number, false));
    assert_eq!(s.required_fields().len(), 1);
    assert_eq!(s.required_fields()[0].name, "title");
}

#[test]
fn to_json_schema_includes_required() {
    let s = OutputSchema::new("task").add_field(FieldSchema::new("title", JsonType::String, true));
    let json = s.to_json_schema();
    assert!(json["required"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("title")));
}

#[test]
fn to_json_schema_types_correct() {
    let s = OutputSchema::new("t").add_field(FieldSchema::new("n", JsonType::Number, false));
    let json = s.to_json_schema();
    assert_eq!(json["properties"]["n"]["type"], "number");
}
