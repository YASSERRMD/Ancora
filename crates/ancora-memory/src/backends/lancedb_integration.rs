/// Integration tests for the LanceDB backend.
///
/// Set `LANCEDB_DIR` to run against a real local LanceDB directory:
///   LANCEDB_DIR=/tmp/test_lancedb cargo test -p ancora-memory -- --ignored lancedb
///
/// All tests are `#[ignore]` so CI passes without any setup.

#[cfg(test)]
mod lancedb_integration {
    use crate::backends::lancedb::*;

    fn test_dir() -> Option<String> {
        std::env::var("LANCEDB_DIR").ok()
    }

    fn cfg() -> Option<LanceDbConfig> {
        test_dir().map(LanceDbConfig::local)
    }

    #[test]
    #[ignore]
    fn integration_local_open_directory() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        println!("Would open LanceDB at {}", cfg.path.uri());
    }

    #[test]
    #[ignore]
    fn integration_create_table_with_schema() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let schema = table_schema(128, &[ColumnDef::new("year", column_type::INT64)]);
        println!(
            "Would create table at {} with schema {schema}",
            cfg.path.uri()
        );
    }

    #[test]
    #[ignore]
    fn integration_add_rows_and_search() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let data = vec![(
            1i64,
            vec![0.1f32; 128],
            serde_json::json!({"title": "doc 1"}),
        )];
        let batch = rows(&data);
        let q = VectorQuery::new("docs", vec![0.1f32; 128], 5);
        println!(
            "Would add {} rows to {}, then search",
            batch.len(),
            cfg.path.uri()
        );
        println!("query: {}", q.to_json());
    }

    #[test]
    #[ignore]
    fn integration_delete_by_predicate() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let pred = delete_predicate("docs", "year < 2020");
        println!("Would delete from {} with {pred}", cfg.path.uri());
    }

    #[test]
    #[ignore]
    fn integration_version_checkout() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let vc = VersionCheckout::new("docs", 1);
        println!(
            "Would checkout version {} from {}",
            vc.version,
            cfg.path.uri()
        );
    }

    #[test]
    #[ignore]
    fn integration_object_storage_s3() {
        let s3_cfg = LanceDbConfig::s3("s3://bucket/lancedb", "us-east-1");
        println!("Would open LanceDB at {}", s3_cfg.path.uri());
    }
}
