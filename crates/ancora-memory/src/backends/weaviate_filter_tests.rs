/// Where-filter translation tests for the Weaviate GraphQL layer.
/// All offline.

#[cfg(test)]
mod weaviate_filter_tests {
    use crate::backends::weaviate::*;

    #[test]
    fn filter_text_equal_has_value_text() {
        let f = where_filter_text("source", "Equal", "wiki");
        assert_eq!(f["valueText"], "wiki");
        assert_eq!(f["operator"], "Equal");
    }

    #[test]
    fn filter_int_greater_than() {
        let f = where_filter_int("year", "GreaterThan", 2020);
        assert_eq!(f["valueInt"], 2020);
        assert_eq!(f["operator"], "GreaterThan");
    }

    #[test]
    fn filter_bool_sets_equal_operator() {
        let f = where_filter_bool("active", true);
        assert_eq!(f["operator"], "Equal");
        assert_eq!(f["valueBoolean"], true);
    }

    #[test]
    fn filter_and_has_and_operator() {
        let a = where_filter_text("lang", "Equal", "en");
        let b = where_filter_int("year", "GreaterThan", 2020);
        let compound = where_filter_and(&[a, b]);
        assert_eq!(compound["operator"], "And");
        assert_eq!(compound["operands"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_or_has_or_operator() {
        let a = where_filter_text("tag", "Equal", "news");
        let b = where_filter_text("tag", "Equal", "blog");
        let compound = where_filter_or(&[a, b]);
        assert_eq!(compound["operator"], "Or");
        assert_eq!(compound["operands"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_path_is_array() {
        let f = where_filter_text("source", "Equal", "x");
        assert!(f["path"].is_array(), "path must be array: {f}");
        assert_eq!(f["path"][0], "source");
    }

    #[test]
    fn filter_not_equal_text() {
        let f = where_filter_text("status", "NotEqual", "deleted");
        assert_eq!(f["operator"], "NotEqual");
    }

    #[test]
    fn filter_triple_and_all_three_operands() {
        let a = where_filter_text("a", "Equal", "x");
        let b = where_filter_int("b", "GreaterThan", 0);
        let c = where_filter_bool("c", true);
        let compound = where_filter_and(&[a, b, c]);
        assert_eq!(compound["operands"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn filter_nested_and_in_or() {
        let inner_and = where_filter_and(&[
            where_filter_text("x", "Equal", "a"),
            where_filter_int("y", "GreaterThan", 5),
        ]);
        let outer = where_filter_or(&[inner_and, where_filter_text("z", "Equal", "b")]);
        assert_eq!(outer["operator"], "Or");
        assert_eq!(outer["operands"].as_array().unwrap().len(), 2);
    }
}
