/// Version checkout and time-travel tests for the LanceDB backend.
/// All offline.

#[cfg(test)]
mod lancedb_version_tests {
    use crate::backends::lancedb::*;

    #[test]
    fn version_checkout_json_has_version_and_table() {
        let vc = VersionCheckout::new("docs", 3);
        let j = vc.to_json();
        assert_eq!(j["version"], 3);
        assert_eq!(j["table"], "docs");
    }

    #[test]
    fn checkout_as_of_includes_unix_timestamp() {
        let j = checkout_as_of("docs", 1720000000);
        assert_eq!(j["as_of"], 1720000000u64);
        assert_eq!(j["table"], "docs");
    }

    #[test]
    fn restore_version_json_has_correct_key() {
        let j = restore_version("docs", 5);
        assert_eq!(j["restore_to"], 5);
    }

    #[test]
    fn parse_version_extracts_from_metadata() {
        let body = serde_json::json!({ "version": 12 });
        assert_eq!(parse_version(&body), 12);
    }

    #[test]
    fn parse_version_defaults_to_zero_on_missing() {
        let body = serde_json::json!({});
        assert_eq!(parse_version(&body), 0);
    }

    #[test]
    fn version_checkout_table_name_preserved() {
        let vc = VersionCheckout::new("my_table", 0);
        assert_eq!(vc.table, "my_table");
    }

    #[test]
    fn restore_and_checkout_are_distinct_operations() {
        let checkout = checkout_as_of("t", 100);
        let restore = restore_version("t", 1);
        assert!(checkout.get("as_of").is_some(), "checkout has as_of");
        assert!(restore.get("restore_to").is_some(), "restore has restore_to");
    }
}
