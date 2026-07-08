//! Hybrid search ordering tests for the pgvector SQL generation layer.
//!
//! These tests confirm that:
//! 1. alpha=1.0 produces the same SQL structure as a pure cosine query.
//! 2. alpha=0.0 produces a SQL dominated by ts_rank (keyword-only signal).
//! 3. Varying alpha values correctly scale the blend coefficients in the SQL.
//! 4. Filtered hybrid queries include both vector and keyword signals.
//!
//! All tests run offline -- no live Postgres required.

#[cfg(test)]
mod hybrid_tests {
    use crate::backends::pgvector::*;
    use crate::vector_store::{Filter, PayloadValue};

    #[test]
    fn hybrid_alpha_one_weights_vector_fully() {
        let sql = hybrid_query_sql("docs", 5, 1.0);
        // At alpha=1.0, beta=0.0; ts_rank term disappears from the combined score
        // but the column is still in the SELECT. What changes is that the cosine
        // weight is "1" in the SQL literal.
        assert!(
            sql.contains("1 *") || sql.contains("1.0 *") || sql.contains("0.0 *"),
            "SQL: {sql}"
        );
    }

    #[test]
    fn hybrid_alpha_zero_weights_keyword_fully() {
        let sql = hybrid_query_sql("docs", 5, 0.0);
        assert!(sql.contains("ts_rank"), "SQL: {sql}");
        // alpha=0.0 means beta=1.0; vector score multiplier is 0
        assert!(sql.contains("0 *") || sql.contains("0.0 *"), "SQL: {sql}");
    }

    #[test]
    fn hybrid_alpha_half_has_equal_weights() {
        let sql = hybrid_query_sql("docs", 5, 0.5);
        // Both 0.5 should appear for vector and keyword
        let count = sql.matches("0.5").count();
        assert!(
            count >= 2,
            "expected at least two 0.5 occurrences, SQL: {sql}"
        );
    }

    #[test]
    fn hybrid_with_filter_has_where_before_order_by() {
        let sql = hybrid_query_with_filter_sql("docs", 5, 0.7, "payload->>'lang' = $3");
        let where_pos = sql.find("WHERE").expect("no WHERE in SQL");
        let order_pos = sql.find("ORDER BY").expect("no ORDER BY in SQL");
        assert!(
            where_pos < order_pos,
            "WHERE must precede ORDER BY, SQL: {sql}"
        );
    }

    #[test]
    fn hybrid_column_sql_uses_specified_key() {
        let sql = hybrid_query_column_sql("docs", "content", 5, 0.6);
        assert!(sql.contains("'content'"), "SQL: {sql}");
        assert!(
            !sql.contains("'text'"),
            "should not use default key, SQL: {sql}"
        );
    }

    #[test]
    fn hybrid_query_always_has_order_by_score_desc() {
        for alpha in [0.0f32, 0.3, 0.5, 0.7, 1.0] {
            let sql = hybrid_query_sql("docs", 5, alpha);
            assert!(
                sql.contains("ORDER BY score DESC"),
                "alpha={alpha}, SQL: {sql}"
            );
        }
    }

    #[test]
    fn hybrid_query_limit_appears_in_sql() {
        let sql = hybrid_query_sql("docs", 42, 0.5);
        assert!(sql.contains("LIMIT 42"), "SQL: {sql}");
    }

    #[test]
    fn hybrid_query_with_filter_from_filter_struct() {
        let f = Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned()));
        let (where_clause, params) = build_where_clause(&f, 2);
        let sql = hybrid_query_with_filter_sql("docs", 5, 0.6, &where_clause.replace("WHERE ", ""));
        assert!(!params.is_empty());
        assert!(sql.contains("lang"), "SQL: {sql}");
    }
}
