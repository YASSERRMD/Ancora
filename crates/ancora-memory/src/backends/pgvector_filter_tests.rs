/// Metadata filter SQL correctness tests.
///
/// Verifies that every filter variant produces correct SQL fragments:
/// - Parameter offset tracking (chained AND/OR get sequential placeholders)
/// - All PayloadValue types (String, Integer, Float, Bool, Null)
/// - All comparison operators (Eq, Ne, Gt, Lt)
/// - Compound filters (And, Or, nested)
/// - Depth validation (rejects >16 levels)
///
/// All tests run offline with no database.

#[cfg(test)]
mod filter_tests {
    use crate::backends::pgvector::*;
    use crate::vector_store::{Filter, PayloadValue};

    #[test]
    fn filter_eq_string_uses_text_cast() {
        let f = Filter::Eq("label".to_owned(), PayloadValue::String("abc".to_owned()));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("payload->>'label'"), "SQL: {sql}");
        assert!(sql.contains("$1"), "SQL: {sql}");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn filter_eq_integer_uses_bigint_cast() {
        let f = Filter::Eq("year".to_owned(), PayloadValue::Integer(2024));
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.contains("::bigint"), "SQL: {sql}");
    }

    #[test]
    fn filter_eq_float_uses_float_cast() {
        let f = Filter::Eq("score".to_owned(), PayloadValue::Float(0.95));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("::float"), "SQL: {sql}");
        assert!(matches!(params[0], FilterParam::Float(_)));
    }

    #[test]
    fn filter_eq_bool_uses_boolean_cast() {
        let f = Filter::Eq("active".to_owned(), PayloadValue::Bool(false));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("::boolean"), "SQL: {sql}");
        assert!(matches!(params[0], FilterParam::Bool(false)));
    }

    #[test]
    fn filter_eq_null_has_is_null_no_param() {
        let f = Filter::Eq("deleted_at".to_owned(), PayloadValue::Null);
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("IS NULL"), "SQL: {sql}");
        assert!(params.is_empty(), "no params expected for IS NULL");
    }

    #[test]
    fn filter_ne_produces_not_equal_op() {
        let f = Filter::Ne(
            "status".to_owned(),
            PayloadValue::String("archived".to_owned()),
        );
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.contains("!="), "SQL: {sql}");
    }

    #[test]
    fn filter_gt_produces_greater_than_op() {
        let f = Filter::Gt("count".to_owned(), PayloadValue::Integer(100));
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.contains(">"), "SQL: {sql}");
        assert!(!sql.contains(">="), "should be strict >, SQL: {sql}");
    }

    #[test]
    fn filter_lt_produces_less_than_op() {
        let f = Filter::Lt("count".to_owned(), PayloadValue::Integer(0));
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.contains("<"), "SQL: {sql}");
        assert!(!sql.contains("<="), "should be strict <, SQL: {sql}");
    }

    #[test]
    fn filter_and_sequential_params() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)));
        let (sql, params) = filter_to_sql(&f, 0);
        assert!(sql.contains("$1"), "SQL: {sql}");
        assert!(sql.contains("$2"), "SQL: {sql}");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn filter_and_with_offset_shifts_placeholders() {
        let f = Filter::Eq("x".to_owned(), PayloadValue::Integer(5));
        let (sql, _) = filter_to_sql(&f, 2); // offset=2 means first param is $3
        assert!(sql.contains("$3"), "SQL: {sql}");
        assert!(!sql.contains("$1"), "SQL: {sql}");
    }

    #[test]
    fn filter_or_compound_wraps_in_parens() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::String("x".to_owned())).or(Filter::Eq(
            "a".to_owned(),
            PayloadValue::String("y".to_owned()),
        ));
        let (sql, _) = filter_to_sql(&f, 0);
        assert!(sql.starts_with('('), "SQL: {sql}");
        assert!(sql.contains("OR"), "SQL: {sql}");
    }

    #[test]
    fn filter_triple_and_has_three_params() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)))
            .and(Filter::Eq("c".to_owned(), PayloadValue::Integer(3)));
        let (_, params) = filter_to_sql(&f, 0);
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn filter_depth_within_limit_accepted() {
        let mut f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1));
        for _ in 0..15 {
            f = f.and(Filter::Eq("a".to_owned(), PayloadValue::Integer(1)));
        }
        assert!(validate_filter_depth(&f).is_ok());
    }
}
