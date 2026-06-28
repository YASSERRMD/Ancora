use crate::entry_schema::{EntrySchema, FieldType, SchemaField};
use crate::metadata::{Author, License, Metadata, Version};
use crate::tool_entry::{ToolEffect, ToolEntry};
use crate::validation::validate_tool;

fn make_metadata() -> Metadata {
    Metadata::new(
        Version::new(1, 0, 0),
        Author::new("YASSERRMD"),
        License::apache2(),
    )
}

#[test]
fn valid_tool_produces_no_errors() {
    let entry = ToolEntry::new(
        "calculator",
        "Calculator",
        "Performs arithmetic operations.",
        ToolEffect::None,
        make_metadata(),
    );
    let errors = validate_tool(&entry);
    assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
}

#[test]
fn tool_with_schema_fields_validates() {
    let schema = EntrySchema::new().with_field(SchemaField::new("query", FieldType::String, true));
    let entry = ToolEntry::new(
        "search-tool",
        "Search",
        "Searches the web.",
        ToolEffect::ReadOnly,
        make_metadata(),
    )
    .with_input_schema(schema);
    assert!(entry.is_valid());
}

#[test]
fn empty_id_marks_entry_invalid() {
    let entry = ToolEntry::new(
        "",
        "Calculator",
        "A tool.",
        ToolEffect::None,
        make_metadata(),
    );
    assert!(!entry.is_valid());
}
