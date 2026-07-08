//! Boolean expression filter correctness tests for the Milvus backend.
//! All offline.

#![cfg(test)]

use crate::backends::milvus::*;

#[test]
fn expr_eq_str_produces_quoted_value() {
    let e = expr_eq_str("status", "active");
    assert_eq!(e, r#"status == "active""#);
}

#[test]
fn expr_eq_int_produces_unquoted_value() {
    let e = expr_eq_int("score", 42);
    assert_eq!(e, "score == 42");
}

#[test]
fn expr_gt_produces_greater_than() {
    let e = expr_gt("year", 2020);
    assert_eq!(e, "year > 2020");
}

#[test]
fn expr_lt_produces_less_than() {
    let e = expr_lt("rank", 100);
    assert_eq!(e, "rank < 100");
}

#[test]
fn expr_in_ints_wraps_list_correctly() {
    let e = expr_in_ints("category", &[1, 2, 3]);
    assert_eq!(e, "category in [1, 2, 3]");
}

#[test]
fn expr_and_wraps_both_sides() {
    let e = expr_and("a > 0", "b < 10");
    assert_eq!(e, "(a > 0) and (b < 10)");
}

#[test]
fn expr_or_wraps_both_sides() {
    let e = expr_or("tag == \"x\"", "tag == \"y\"");
    assert_eq!(e, r#"(tag == "x") or (tag == "y")"#);
}

#[test]
fn escape_string_escapes_backslash() {
    let s = escape_string(r#"C:\path"#);
    assert!(s.contains(r#"\\"#), "got: {s}");
}

#[test]
fn search_with_filter_propagates_expr() {
    let body = search_with_filter_body(
        "col",
        &[0.1f32],
        5,
        metric_type::COSINE,
        "status == \"ok\"",
        &["payload"],
    );
    assert_eq!(body["filter"], "status == \"ok\"");
}

#[test]
fn query_with_complex_expr_is_set() {
    let expr = expr_and(&expr_gt("score", 5), &expr_eq_str("tag", "ml"));
    let body = query_body("col", &expr, &["id"]);
    let stored = body["filter"].as_str().unwrap();
    assert!(
        stored.contains("score") && stored.contains("ml"),
        "expr: {stored}"
    );
}

#[test]
fn delete_by_expr_accepts_complex_filter() {
    let expr = expr_or(&expr_lt("score", 1), &expr_eq_str("status", "deleted"));
    let body = delete_by_expr_body("col", &expr);
    assert!(
        body["filter"].as_str().unwrap().contains("or"),
        "body: {body}"
    );
}

#[test]
fn expr_range_builds_ge_le_expr() {
    let e = expr_range("year", 2020, 2024);
    assert_eq!(e, "year >= 2020 and year <= 2024");
}

#[test]
fn expr_ne_int_builds_not_equal() {
    let e = expr_ne_int("status", 0);
    assert_eq!(e, "status != 0");
}

#[test]
fn expr_ne_str_builds_not_equal_string() {
    let e = expr_ne_str("label", "deleted");
    assert_eq!(e, r#"label != "deleted""#);
}

#[test]
fn expr_not_wraps_in_parentheses() {
    let e = expr_not("active == false");
    assert_eq!(e, "not (active == false)");
}
