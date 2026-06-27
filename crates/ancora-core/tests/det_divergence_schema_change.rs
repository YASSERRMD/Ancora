/// Determinism: divergence is detected when schema changes alter input structure.
/// If the input schema for an activity changes, the recorded input_json becomes incompatible
/// with the new schema, which is detected as a divergence.
use ancora_core::replay::detect_divergence;
use serde_json::Value;

fn check_schema_compat(recorded_input: &str, new_schema_required_field: &str) -> bool {
    let v: Value = serde_json::from_str(recorded_input).unwrap();
    v.get(new_schema_required_field).is_some()
}

#[test] fn old_schema_missing_new_required_field_is_incompatible() {
    let old_input = r#"{"query":"search","top_k":5}"#;
    let new_required = "embedding_model";
    assert!(!check_schema_compat(old_input, new_required), "old schema must be incompatible");
}

#[test] fn old_schema_with_new_required_field_is_compatible() {
    let old_input = r#"{"query":"search","top_k":5,"embedding_model":"ada-002"}"#;
    let new_required = "embedding_model";
    assert!(check_schema_compat(old_input, new_required));
}

#[test] fn detect_divergence_catches_schema_driven_key_change() {
    let expected = vec!["retrieve:schema-v1".into()];
    let observed_after_schema_change = vec!["retrieve:schema-v2".into()];
    assert!(detect_divergence(&expected, &observed_after_schema_change).is_err());
}

#[test] fn detect_divergence_no_error_when_schema_unchanged() {
    let keys = vec!["retrieve:schema-v1".into(), "process:schema-v1".into()];
    assert!(detect_divergence(&keys, &keys).is_ok());
}

#[test] fn schema_version_in_activity_key_enables_detection() {
    let v1_keys = vec!["compute:v1".into()];
    let v2_keys = vec!["compute:v2".into()];
    assert!(detect_divergence(&v1_keys, &v2_keys).is_err());
}

#[test] fn empty_schema_change_keys_both_empty_no_divergence() {
    let empty: Vec<String> = vec![];
    assert!(detect_divergence(&empty, &empty).is_ok());
}
