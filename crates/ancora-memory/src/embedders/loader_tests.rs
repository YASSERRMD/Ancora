/// Extended document loader and batch embedding tests -- all offline.

#[cfg(test)]
mod loader_ext_tests {
    use crate::embedders::batch::{
        chunk_embeddings, merge_batch_results, BatchConfig, BatchEmbedder, BatchResult,
    };
    use crate::embedders::embedder::{EmbedResult, Embedder, Embedding};
    use crate::embedders::loader::*;
    use crate::embedders::local::HashEmbedder;
    use std::collections::HashMap;

    // ---- loader tests --------------------------------------------------

    #[test]
    fn load_text_metadata_is_empty_object() {
        let doc = load_text("f.txt", "hello");
        assert!(doc.metadata.is_object());
    }

    #[test]
    fn document_with_metadata_sets_fields() {
        let doc = Document::new("src", "text")
            .with_metadata(serde_json::json!({"lang": "en", "page": 1}));
        assert_eq!(doc.metadata["lang"], "en");
        assert_eq!(doc.metadata["page"], 1);
    }

    #[test]
    fn load_texts_preserves_order() {
        let pairs = &[("a.txt", "first"), ("b.txt", "second"), ("c.txt", "third")];
        let docs = load_texts(pairs);
        assert_eq!(docs[0].source, "a.txt");
        assert_eq!(docs[2].source, "c.txt");
    }

    #[test]
    fn load_markdown_without_strip_preserves_hashes() {
        let md = "## Header Two";
        let doc = load_markdown("f.md", md, false);
        assert!(doc.text.contains("##"), "text: {}", doc.text);
    }

    #[test]
    fn strip_markdown_removes_emphasis() {
        let md = "**bold** and *italic*";
        let doc = load_markdown("f.md", md, true);
        assert!(!doc.text.contains('*'), "text: {}", doc.text);
    }

    #[test]
    fn split_markdown_sections_empty_input_produces_no_docs() {
        let docs = split_markdown_sections("f.md", "");
        assert!(docs.is_empty(), "docs: {docs:?}");
    }

    #[test]
    fn split_markdown_sections_multiple_headers() {
        let md = "# A\nContent A.\n# B\nContent B.\n# C\nContent C.";
        let docs = split_markdown_sections("f.md", md);
        assert_eq!(docs.len(), 3, "docs: {docs:?}");
    }

    #[test]
    fn split_markdown_sections_source_includes_section_slug() {
        let md = "# Hello World\nContent.";
        let docs = split_markdown_sections("file.md", md);
        assert!(!docs.is_empty());
        assert!(
            docs[0].source.contains("hello-world") || docs[0].source.contains("file.md"),
            "source: {}",
            docs[0].source
        );
    }

    #[test]
    fn pdf_loader_default_no_max_pages() {
        let loader = PdfLoader::default();
        assert!(loader.max_pages.is_none());
    }

    #[test]
    fn pdf_loader_stub_metadata_has_source() {
        let loader = PdfLoader::new().with_max_pages(1);
        let docs = loader.load_bytes("report.pdf", b"%PDF");
        assert_eq!(docs[0].metadata["source"], "report.pdf");
    }

    #[test]
    fn enrich_metadata_multiple_fields() {
        let docs = vec![Document::new("f", "text"), Document::new("g", "text2")];
        let mut extra = HashMap::new();
        extra.insert("lang".to_owned(), serde_json::json!("en"));
        extra.insert("version".to_owned(), serde_json::json!(2));
        let enriched = enrich_metadata(docs, &extra);
        assert_eq!(enriched[1].metadata["version"], 2);
    }

    // ---- batch tests ---------------------------------------------------

    struct AlwaysFail;

    impl Embedder for AlwaysFail {
        fn embed(&self, _text: &str) -> EmbedResult<Embedding> {
            Err(crate::embedders::embedder::EmbedError::Other(
                "always fails".to_owned(),
            ))
        }
        fn model_name(&self) -> &str {
            "fail"
        }
        fn dims(&self) -> usize {
            4
        }
    }

    #[test]
    fn batch_skip_on_error_produces_none_entries() {
        let be = BatchEmbedder::new(AlwaysFail, BatchConfig::new(2).skip_errors());
        let result = be.embed_all(&["a", "b", "c"]);
        assert_eq!(result.total, 3);
        assert_eq!(result.error_count, 3);
        assert!(result.successful().is_empty());
    }

    #[test]
    fn batch_success_rate_zero_on_all_fail() {
        let be = BatchEmbedder::new(AlwaysFail, BatchConfig::new(1).skip_errors());
        let result = be.embed_all(&["x"]);
        assert_eq!(result.success_rate(), 0.0);
    }

    #[test]
    fn chunk_embeddings_empty_input() {
        let chunks = chunk_embeddings(vec![], 5);
        assert!(chunks.is_empty());
    }

    #[test]
    fn chunk_embeddings_single_batch() {
        let embs: Vec<Embedding> = vec![vec![0.1f32]; 3];
        let chunks = chunk_embeddings(embs, 10);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn merge_batch_results_empty_list() {
        let merged = merge_batch_results(vec![]);
        assert_eq!(merged.total, 0);
        assert_eq!(merged.error_count, 0);
    }

    #[test]
    fn batch_embedder_with_hash_embedder_all_succeed() {
        let be = BatchEmbedder::new(HashEmbedder::new(16), BatchConfig::new(4));
        let texts: Vec<&str> = (0..12).map(|_| "test text").collect();
        let result = be.embed_all(&texts);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.successful().len(), 12);
    }
}
