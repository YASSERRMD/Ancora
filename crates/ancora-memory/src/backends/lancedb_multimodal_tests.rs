//! Multimodal column support tests for the LanceDB backend.
//! All offline.

#![cfg(test)]

use crate::backends::lancedb::*;

#[test]
fn multimodal_row_has_text_embedding() {
    let r = multimodal_row(1, vec![0.1f32; 4], vec![0.2f32; 4], serde_json::json!({}));
    assert!(r["text_embedding"].is_array());
}

#[test]
fn multimodal_row_has_image_embedding() {
    let r = multimodal_row(1, vec![0.1f32; 4], vec![0.2f32; 4], serde_json::json!({}));
    assert!(r["image_embedding"].is_array());
}

#[test]
fn multimodal_row_text_embedding_length_preserved() {
    let r = multimodal_row(1, vec![0.5f32; 8], vec![0.0f32; 4], serde_json::json!({}));
    assert_eq!(r["text_embedding"].as_array().unwrap().len(), 8);
}

#[test]
fn multimodal_row_image_embedding_length_preserved() {
    let r = multimodal_row(1, vec![0.0f32; 4], vec![0.9f32; 16], serde_json::json!({}));
    assert_eq!(r["image_embedding"].as_array().unwrap().len(), 16);
}

#[test]
fn multimodal_row_has_id() {
    let r = multimodal_row(99, vec![0.1f32], vec![0.2f32], serde_json::json!({}));
    assert_eq!(r["id"], 99);
}

#[test]
fn row_with_columns_adds_custom_fields() {
    let r = row_with_columns(
        1,
        vec![0.1f32],
        serde_json::json!({}),
        serde_json::json!({"category": "image", "width": 1024}),
    );
    assert_eq!(r["category"], "image");
    assert_eq!(r["width"], 1024);
}

#[test]
fn table_schema_supports_multiple_extra_columns() {
    let cols = vec![
        ColumnDef::new("width", column_type::INT64),
        ColumnDef::new("height", column_type::INT64),
        ColumnDef::new("format", column_type::UTF8),
    ];
    let schema = table_schema(256, &cols);
    let col_list = schema["columns"].as_array().unwrap();
    // base (id, embedding, payload) + 3 extra = 6
    assert_eq!(col_list.len(), 6);
}

#[test]
fn ann_index_supports_multiple_metrics() {
    for metric in &["cosine", "l2", "dot"] {
        let idx = AnnIndex::new(256, 16).metric(*metric);
        assert_eq!(idx.to_json()["metric"], *metric);
    }
}
