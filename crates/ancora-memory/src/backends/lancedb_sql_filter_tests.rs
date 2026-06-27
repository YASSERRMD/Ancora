/// SQL filter correctness tests for the LanceDB backend.
/// All offline.

#[cfg(test)]
mod lancedb_sql_filter_tests {
    use crate::backends::lancedb::*;

    #[test]
    fn sql_eq_str_single_quotes_value() {
        assert_eq!(sql_eq_str("tag", "rust"), "tag = 'rust'");
    }

    #[test]
    fn sql_eq_str_escapes_embedded_single_quote() {
        let e = sql_eq_str("name", "O'Brien");
        assert_eq!(e, "name = 'O''Brien'");
    }

    #[test]
    fn sql_eq_int_no_quotes() {
        assert_eq!(sql_eq_int("year", 2024), "year = 2024");
    }

    #[test]
    fn sql_gt_operator() {
        assert_eq!(sql_gt("score", 5), "score > 5");
    }

    #[test]
    fn sql_lt_operator() {
        assert_eq!(sql_lt("rank", 100), "rank < 100");
    }

    #[test]
    fn sql_is_null_format() {
        assert_eq!(sql_is_null("category"), "category IS NULL");
    }

    #[test]
    fn sql_is_not_null_format() {
        assert_eq!(sql_is_not_null("category"), "category IS NOT NULL");
    }

    #[test]
    fn sql_and_combines_with_parentheses() {
        let e = sql_and("a = 1", "b = 2");
        assert_eq!(e, "(a = 1) AND (b = 2)");
    }

    #[test]
    fn sql_or_combines_with_parentheses() {
        let e = sql_or("a = 1", "b = 2");
        assert_eq!(e, "(a = 1) OR (b = 2)");
    }

    #[test]
    fn sql_in_ints_format() {
        let e = sql_in_ints("category_id", &[10, 20, 30]);
        assert_eq!(e, "category_id IN (10, 20, 30)");
    }

    #[test]
    fn nested_and_or_expression() {
        let inner = sql_or(&sql_eq_int("type", 1), &sql_eq_int("type", 2));
        let outer = sql_and(&sql_gt("score", 0), &inner);
        assert!(outer.contains("AND"), "outer: {outer}");
        assert!(outer.contains("OR"), "outer: {outer}");
    }

    #[test]
    fn vector_query_filter_uses_sql_helpers() {
        let sql = sql_and(&sql_gt("year", 2020), &sql_eq_str("lang", "en"));
        let q = VectorQuery::new("docs", vec![0.1f32], 5).filter(sql.clone());
        assert_eq!(q.to_json()["filter"], sql);
    }
}
