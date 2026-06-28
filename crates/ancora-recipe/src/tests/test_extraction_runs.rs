use crate::data_extraction::{build, ExtractedRecord, Schema, SchemaField};
use crate::params::ParamSet;

#[test]
fn extraction_recipe_builds() {
    let mut ps = ParamSet::new();
    ps.set("schema", "invoice");
    ps.set("output_format", "csv");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "data-extraction");
}

#[test]
fn extraction_recipe_has_four_steps() {
    let ps = ParamSet::default();
    let r = build(&ps);
    assert_eq!(r.step_count(), 4);
}

#[test]
fn schema_with_optional_fields_passes_empty_record() {
    let schema = Schema::new(vec![SchemaField { name: "optional".into(), required: false }]);
    let record = ExtractedRecord::default();
    assert!(schema.validate(&record).is_ok());
}

#[test]
fn extracted_record_get_returns_correct_value() {
    let mut r = ExtractedRecord::default();
    r.set("vendor", "Acme Corp");
    assert_eq!(r.get("vendor"), Some("Acme Corp"));
    assert_eq!(r.get("total"), None);
}
