/// Filter translation correctness tests for the Qdrant REST layer.
///
/// Verifies every Filter variant produces the correct Qdrant filter JSON.
/// All offline.

#[cfg(test)]
mod qdrant_filter_tests {
    use crate::backends::qdrant::filter_to_qdrant;
    use crate::vector_store::{Filter, PayloadValue};

    #[test]
    fn filter_eq_string_match_value() {
        let f = Filter::Eq("source".to_owned(), PayloadValue::String("wiki".to_owned()));
        let json = filter_to_qdrant(&f);
        assert_eq!(json["must"][0]["key"], "source");
        assert_eq!(json["must"][0]["match"]["value"], "wiki");
    }

    #[test]
    fn filter_eq_integer_match_value() {
        let f = Filter::Eq("year".to_owned(), PayloadValue::Integer(2024));
        let json = filter_to_qdrant(&f);
        assert_eq!(json["must"][0]["match"]["value"], 2024);
    }

    #[test]
    fn filter_eq_bool_match_value() {
        let f = Filter::Eq("active".to_owned(), PayloadValue::Bool(true));
        let json = filter_to_qdrant(&f);
        assert_eq!(json["must"][0]["match"]["value"], true);
    }

    #[test]
    fn filter_ne_uses_must_not() {
        let f = Filter::Ne(
            "status".to_owned(),
            PayloadValue::String("deleted".to_owned()),
        );
        let json = filter_to_qdrant(&f);
        assert!(json["must_not"].is_array(), "json: {json}");
        assert_eq!(json["must_not"][0]["key"], "status");
    }

    #[test]
    fn filter_gt_uses_range_gt() {
        let f = Filter::Gt("count".to_owned(), PayloadValue::Integer(100));
        let json = filter_to_qdrant(&f);
        assert!(json["must"][0]["range"]["gt"].is_number());
        assert_eq!(json["must"][0]["range"]["gt"], 100);
    }

    #[test]
    fn filter_lt_uses_range_lt() {
        let f = Filter::Lt("score".to_owned(), PayloadValue::Float(0.5));
        let json = filter_to_qdrant(&f);
        let lt_val = json["must"][0]["range"]["lt"].as_f64().unwrap();
        assert!((lt_val - 0.5).abs() < 0.001);
    }

    #[test]
    fn filter_and_two_conditions_both_in_must() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)));
        let json = filter_to_qdrant(&f);
        let must = json["must"].as_array().unwrap();
        assert_eq!(must.len(), 2);
        let keys: Vec<&str> = must.iter().map(|m| m["key"].as_str().unwrap()).collect();
        assert!(keys.contains(&"a"));
        assert!(keys.contains(&"b"));
    }

    #[test]
    fn filter_or_uses_should() {
        let f = Filter::Eq("tag".to_owned(), PayloadValue::String("news".to_owned())).or(
            Filter::Eq("tag".to_owned(), PayloadValue::String("blog".to_owned())),
        );
        let json = filter_to_qdrant(&f);
        assert!(json["should"].is_array(), "json: {json}");
        assert_eq!(json["should"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_triple_and_all_three_in_must() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)))
            .and(Filter::Eq("c".to_owned(), PayloadValue::Integer(3)));
        let json = filter_to_qdrant(&f);
        let must = json["must"].as_array().unwrap();
        assert_eq!(must.len(), 3);
    }

    #[test]
    fn filter_and_then_or_nests_correctly() {
        let inner = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)));
        let f = inner.or(Filter::Eq("c".to_owned(), PayloadValue::Integer(3)));
        let json = filter_to_qdrant(&f);
        assert!(json["should"].is_array(), "json: {json}");
    }

    #[test]
    fn filter_null_value_produces_null_match() {
        let f = Filter::Eq("field".to_owned(), PayloadValue::Null);
        let json = filter_to_qdrant(&f);
        assert!(json["must"][0]["match"]["value"].is_null(), "json: {json}");
    }
}
