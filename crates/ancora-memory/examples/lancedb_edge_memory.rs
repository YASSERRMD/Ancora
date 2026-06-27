/// LanceDB edge-memory example.
///
/// Demonstrates how to use LanceDB as an embedded, zero-server vector store
/// for single-binary and offline-first deployments. No external process or
/// cloud service is needed.
///
/// Set ANCORA_LANCEDB_DIR to override the default database directory.
/// Without the env var the example uses `./ancora_lancedb`.

use ancora_memory::backends::lancedb::{
    LanceDbConfig, LanceDbPath,
    table_schema, ColumnDef, column_type,
    row, rows, row_with_columns,
    VectorQuery, HybridQuery,
    FullTextIndex, AnnIndex,
    multimodal_row,
    delete_predicate,
    VersionCheckout, checkout_as_of, restore_version,
    sql_and, sql_gt, sql_eq_str,
    parse_results, parse_version,
    detect_storage_type,
    edge_config, edge_default_dir,
};

const TABLE: &str = "documents";
const DIMS: usize = 384;

fn main() {
    println!("=== LanceDB edge-memory example ===\n");

    // 1. Edge config (zero-server)
    let cfg = edge_config();
    println!("Edge DB dir: {}\n", cfg.path.uri());

    // 2. Table schema
    let schema = table_schema(DIMS, &[
        ColumnDef::new("title", column_type::UTF8),
        ColumnDef::new("year", column_type::INT64).required(),
    ]);
    println!("-- Table schema --");
    println!("{}\n", serde_json::to_string_pretty(&schema).unwrap());

    // 3. Row insertion
    let doc_rows = rows(&[
        (1i64, vec![0.1f32; 8], serde_json::json!({"title": "Intro to RAG", "year": 2023})),
        (2i64, vec![0.2f32; 8], serde_json::json!({"title": "Vector search", "year": 2024})),
    ]);
    println!("-- Rows to add ({}) --", doc_rows.len());
    println!("{}\n", serde_json::to_string_pretty(&doc_rows[0]).unwrap());

    // 4. Row with extra columns
    let rich = row_with_columns(
        3, vec![0.3f32; 8],
        serde_json::json!({"title": "LanceDB guide"}),
        serde_json::json!({"year": 2024, "lang": "en"}),
    );
    println!("-- Rich row --");
    println!("{}\n", serde_json::to_string_pretty(&rich).unwrap());

    // 5. Vector search query
    let q = VectorQuery::new(TABLE, vec![0.15f32; 8], 10)
        .metric("cosine")
        .filter(sql_and(&sql_gt("year", 2022), &sql_eq_str("lang", "en")))
        .ef(64)
        .select(&["id", "title", "year"]);
    println!("-- Vector search descriptor --");
    println!("{}\n", serde_json::to_string_pretty(&q.to_json()).unwrap());

    // 6. Hybrid vector + FTS query
    let h = HybridQuery::new(TABLE, vec![0.15f32; 8], "RAG vector search", 5)
        .filter("year > 2021");
    println!("-- Hybrid search descriptor --");
    println!("{}\n", serde_json::to_string_pretty(&h.to_json()).unwrap());

    // 7. Full-text index
    let fts = FullTextIndex::new("title").with_position();
    println!("-- FTS index config --");
    println!("{}\n", serde_json::to_string_pretty(&fts.to_json()).unwrap());

    // 8. ANN index
    let ann = AnnIndex::new(256, 16).metric("cosine");
    println!("-- ANN index config --");
    println!("{}\n", serde_json::to_string_pretty(&ann.to_json()).unwrap());

    // 9. Multimodal row
    let mm = multimodal_row(10, vec![0.1f32; 8], vec![0.9f32; 8], serde_json::json!({"src": "img.png"}));
    println!("-- Multimodal row --");
    println!("{}\n", serde_json::to_string_pretty(&mm).unwrap());

    // 10. Delete by predicate
    let del = delete_predicate(TABLE, "year < 2020");
    println!("-- Delete predicate --");
    println!("{}\n", serde_json::to_string_pretty(&del).unwrap());

    // 11. Version checkout / time-travel
    let vc = VersionCheckout::new(TABLE, 3);
    println!("-- Version checkout --");
    println!("{}\n", serde_json::to_string_pretty(&vc.to_json()).unwrap());

    let ts = checkout_as_of(TABLE, 1720000000);
    println!("-- Checkout as of --");
    println!("{}\n", serde_json::to_string_pretty(&ts).unwrap());

    let rv = restore_version(TABLE, 2);
    println!("-- Restore to version --");
    println!("{}\n", serde_json::to_string_pretty(&rv).unwrap());

    // 12. Parse mock response
    let mock = serde_json::json!({
        "rows": [
            { "id": 1, "_distance": 0.1, "payload": r#"{"title":"Intro"}"# },
        ],
        "version": 5,
    });
    let results = parse_results(&mock);
    println!("-- Parsed {} result(s), version {} --", results.len(), parse_version(&mock));

    // 13. Storage detection
    for path in &["/data/db", "s3://bucket/lancedb", "gs://b/k", "az://c/b"] {
        println!("  {} -> {}", path, detect_storage_type(path));
    }

    println!("\nSet ANCORA_LANCEDB_DIR=/your/path to override the default.");
    let _ = LanceDbConfig::s3("s3://bucket/lancedb", "us-east-1");
    let _ = LanceDbPath::gcs("gs://bucket/lancedb");
    println!("S3 / GCS / Azure paths work the same -- just swap the URI.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_edge_config_is_local() {
        let cfg = edge_config();
        assert!(cfg.path.is_local());
    }

    #[test]
    fn example_schema_has_title_column() {
        let schema = table_schema(DIMS, &[ColumnDef::new("title", column_type::UTF8)]);
        let cols = schema["columns"].as_array().unwrap();
        assert!(cols.iter().any(|c| c["name"] == "title"));
    }

    #[test]
    fn example_edge_default_dir_not_empty() {
        let dir = edge_default_dir();
        assert!(!dir.is_empty());
    }
}
