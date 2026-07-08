//! Redis Vector filter expression and multi-index tests -- all offline.

#![cfg(test)]

use crate::backends::redis_vector::*;

// ---- tag_filter -------------------------------------------------

#[test]
fn tag_filter_single_value() {
    let f = tag_filter("category", "tech");
    assert_eq!(f, "@category:{tech}");
}

#[test]
fn tag_filter_special_characters_in_value() {
    // RediSearch tag escaping is handled by the caller; we just check format.
    let f = tag_filter("lang", "zh-CN");
    assert!(f.contains("zh-CN"), "filter: {f}");
}

// ---- numeric_range -----------------------------------------------

#[test]
fn numeric_range_zero_to_one() {
    let f = numeric_range("score", 0.0, 1.0);
    assert_eq!(f, "@score:[0 1]");
}

#[test]
fn numeric_range_negative_lo() {
    let f = numeric_range("temp", -10.5, 0.0);
    assert!(f.contains("-10.5"), "filter: {f}");
}

#[test]
fn numeric_range_large_values() {
    let f = numeric_range("count", 1_000_000.0, 9_999_999.0);
    assert!(f.starts_with("@count:"), "filter: {f}");
}

// ---- text_match -------------------------------------------------

#[test]
fn text_match_wraps_in_parens() {
    let f = text_match("body", "hello world");
    assert_eq!(f, "@body:(hello world)");
}

// ---- compound pre-filters ---------------------------------------

#[test]
fn compound_tag_and_numeric() {
    let tag = tag_filter("lang", "en");
    let range = numeric_range("score", 0.8, 1.0);
    let compound = format!("({tag}) ({range})");
    assert!(compound.contains("@lang:{en}"), "compound: {compound}");
    assert!(compound.contains("@score:"), "compound: {compound}");
}

#[test]
fn filtered_ann_embeds_compound_filter() {
    let pre = format!(
        "({}) ({})",
        tag_filter("lang", "en"),
        numeric_range("score", 0.8, 1.0)
    );
    let s = SearchArgs::filtered_ann("idx", &pre, "emb", 5);
    let q = s.to_json()["query"].as_str().unwrap().to_owned();
    assert!(q.contains("@lang:{en}"), "q: {q}");
    assert!(q.contains("KNN"), "q: {q}");
}

// ---- CreateIndexArgs extra fields --------------------------------

#[test]
fn extra_text_field_added_to_schema() {
    let idx = CreateIndexArgs::new("docs", "doc:", 128).add_field("title", field_type::TEXT);
    let j = idx.to_json();
    let schema = j["schema"].as_array().unwrap();
    let has_title = schema.iter().any(|v| v.as_str() == Some("title"));
    assert!(has_title, "schema: {schema:?}");
}

#[test]
fn extra_tag_field_added_to_schema() {
    let idx = CreateIndexArgs::new("docs", "doc:", 128).add_field("lang", field_type::TAG);
    let j = idx.to_json();
    let schema = j["schema"].as_array().unwrap();
    let has_tag_type = schema.iter().any(|v| v.as_str() == Some(field_type::TAG));
    assert!(has_tag_type, "schema: {schema:?}");
}

#[test]
fn extra_numeric_field_added() {
    let idx = CreateIndexArgs::new("docs", "doc:", 128).add_field("score", field_type::NUMERIC);
    let j = idx.to_json();
    let schema = j["schema"].as_array().unwrap();
    let has_numeric = schema
        .iter()
        .any(|v| v.as_str() == Some(field_type::NUMERIC));
    assert!(has_numeric, "schema: {schema:?}");
}

// ---- document_key prefix variations ------------------------------

#[test]
fn document_key_numeric_id_padded() {
    let key = document_key("vec", 1);
    assert_eq!(key, "vec:1");
}

#[test]
fn document_key_large_id() {
    let key = document_key("emb", 9_999_999);
    assert_eq!(key, "emb:9999999");
}

// ---- hset_descriptor --------------------------------------------

#[test]
fn hset_descriptor_contains_embedding_dims() {
    let desc = hset_descriptor("doc:1", &[0.1f32, 0.2f32, 0.3f32], serde_json::json!({}));
    assert_eq!(desc["fields"]["embedding_dims"], 3);
}

#[test]
fn hset_descriptor_command_is_hset() {
    let desc = hset_descriptor("doc:2", &[0.5f32], serde_json::json!({"k": "v"}));
    assert_eq!(desc["command"], "HSET");
}

// ---- SearchArgs returns fields -----------------------------------

#[test]
fn search_returns_custom_fields() {
    let s = SearchArgs::ann("idx", "emb", 10).returns(&["score", "title", "lang"]);
    let j = s.to_json();
    let ret = j["return"].as_array().unwrap();
    assert!(ret.contains(&serde_json::json!("title")));
    assert!(ret.contains(&serde_json::json!("lang")));
}

#[test]
fn search_dialect_is_2() {
    let s = SearchArgs::ann("idx", "emb", 5);
    assert_eq!(s.to_json()["DIALECT"], 2);
}

// ---- error edge cases -------------------------------------------

#[test]
fn unknown_redis_error_classified() {
    let e = RedisVectorError::from_redis_err("ERR some unknown error");
    assert!(matches!(e, RedisVectorError::Unknown(_)));
}

#[test]
fn error_wrongtype_not_transient() {
    let e = RedisVectorError::from_redis_err("WRONGTYPE key");
    assert!(!e.is_transient());
}
