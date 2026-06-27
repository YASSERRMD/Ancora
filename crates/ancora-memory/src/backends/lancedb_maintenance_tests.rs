/// Table maintenance and schema evolution tests for the LanceDB backend.
/// All offline.

#[cfg(test)]
mod lancedb_maintenance_tests {
    use crate::backends::lancedb::*;

    #[test]
    fn compact_descriptor_has_operation() {
        let j = compact_descriptor("docs");
        assert_eq!(j["operation"], "compact_files");
        assert_eq!(j["table"], "docs");
    }

    #[test]
    fn cleanup_old_versions_includes_days() {
        let j = cleanup_old_versions_descriptor("docs", 30);
        assert_eq!(j["older_than_days"], 30);
        assert_eq!(j["operation"], "cleanup_old_versions");
    }

    #[test]
    fn optimize_descriptor_has_operation() {
        let j = optimize_descriptor("docs");
        assert_eq!(j["operation"], "optimize");
    }

    #[test]
    fn merge_insert_sets_on_column() {
        let j = merge_insert_descriptor("docs", "id", vec![]);
        assert_eq!(j["on"], "id");
        assert_eq!(j["operation"], "merge_insert");
    }

    #[test]
    fn merge_insert_includes_data_array() {
        let batch = vec![serde_json::json!({"id": 1}), serde_json::json!({"id": 2})];
        let j = merge_insert_descriptor("docs", "id", batch);
        assert_eq!(j["data"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn add_column_descriptor_has_name_and_type() {
        let col = ColumnDef::new("score", column_type::FLOAT32);
        let j = add_column_descriptor("docs", &col, None);
        assert_eq!(j["column"], "score");
        assert_eq!(j["type"], column_type::FLOAT32);
    }

    #[test]
    fn add_column_descriptor_includes_default_when_set() {
        let col = ColumnDef::new("score", column_type::FLOAT32);
        let j = add_column_descriptor("docs", &col, Some(serde_json::json!(0.0)));
        assert_eq!(j["default"], 0.0f64);
    }

    #[test]
    fn drop_column_descriptor_has_correct_fields() {
        let j = drop_column_descriptor("docs", "old_field");
        assert_eq!(j["column"], "old_field");
        assert_eq!(j["operation"], "drop_column");
    }

    #[test]
    fn rename_column_descriptor_has_old_and_new_names() {
        let j = rename_column_descriptor("docs", "text", "body");
        assert_eq!(j["old_name"], "text");
        assert_eq!(j["new_name"], "body");
    }

    #[test]
    fn edge_config_is_local() {
        let cfg = edge_config();
        assert!(cfg.path.is_local());
    }
}
