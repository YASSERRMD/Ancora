//! Extended retrieval pipeline tests -- all offline.

#[cfg(test)]
mod pipeline_ext_tests {
    use crate::embedders::citation::{build_citations, dedup_by_source, filter_by_score};
    use crate::embedders::context::ContextAssembler;
    use crate::embedders::local::{HashEmbedder, TfidfEmbedder};
    use crate::embedders::pipeline::{PipelineConfig, RetrievalPipeline};
    use crate::embedders::rerank::{rrf_fuse, sort_by_score, CosineReranker, ScoredPassage};
    use std::sync::Arc;

    fn hash_pipeline(top_k: usize) -> RetrievalPipeline {
        RetrievalPipeline::new(
            Arc::new(HashEmbedder::new(128)),
            PipelineConfig::new(8, 1, top_k),
        )
    }

    // ---- pipeline tests ------------------------------------------------

    #[test]
    fn pipeline_ingest_returns_chunk_count() {
        let mut p = hash_pipeline(5);
        let count = p
            .ingest(
                "f.txt",
                "word0 word1 word2 word3 word4 word5 word6 word7 word8 word9",
            )
            .unwrap();
        assert!(count >= 1, "expected >=1 chunks, got {count}");
    }

    #[test]
    fn pipeline_passage_count_accumulates() {
        let mut p = hash_pipeline(5);
        let c1 = p
            .ingest("a.txt", "aaaa bbbb cccc dddd eeee ffff gggg hhhh iiii jjjj")
            .unwrap();
        let c2 = p
            .ingest("b.txt", "xxxx yyyy zzzz wwww vvvv uuuu tttt ssss rrrr qqqq")
            .unwrap();
        assert_eq!(p.passage_count(), c1 + c2);
    }

    #[test]
    fn pipeline_query_scores_are_valid_range() {
        let mut p = hash_pipeline(10);
        p.ingest(
            "doc.txt",
            "the quick brown fox jumps over the lazy dog yes it does",
        )
        .unwrap();
        let results = p.query("quick brown fox").unwrap();
        for r in &results {
            assert!(
                r.score >= -1.0 && r.score <= 1.0 + 1e-4,
                "score: {}",
                r.score
            );
        }
    }

    #[test]
    fn pipeline_top_k_respected() {
        let mut p = hash_pipeline(2);
        p.ingest(
            "doc.txt",
            "w0 w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14 w15",
        )
        .unwrap();
        let results = p.query("w0 w1 w2").unwrap();
        assert!(
            results.len() <= 2,
            "top_k=2 but got {} results",
            results.len()
        );
    }

    #[test]
    fn pipeline_citation_source_matches_ingested() {
        let mut p = hash_pipeline(5);
        p.ingest(
            "report.pdf",
            "analysis of quarterly performance metrics data",
        )
        .unwrap();
        let cits = p.query_with_citations("quarterly performance").unwrap();
        if !cits.is_empty() {
            assert_eq!(cits[0].source, "report.pdf");
        }
    }

    #[test]
    fn pipeline_clear_allows_reingest() {
        let mut p = hash_pipeline(5);
        p.ingest("old.txt", "old content that should be cleared")
            .unwrap();
        p.clear();
        p.ingest("new.txt", "new content after clearing the store")
            .unwrap();
        assert!(p.passage_count() > 0);
    }

    // ---- tfidf pipeline -----------------------------------------------

    #[test]
    fn tfidf_pipeline_ingest_and_query() {
        let docs = &[
            "machine learning models",
            "vector search databases",
            "natural language processing",
        ];
        let embedder = TfidfEmbedder::fit(docs, 50);
        let mut p = RetrievalPipeline::new(Arc::new(embedder), PipelineConfig::new(4, 0, 3));
        for (i, doc) in docs.iter().enumerate() {
            p.ingest(&format!("doc{i}.txt"), doc).unwrap();
        }
        let results = p.query("machine learning").unwrap();
        assert!(!results.is_empty(), "expected at least one result");
    }

    // ---- reranker integration ------------------------------------------

    #[test]
    fn cosine_reranker_changes_order() {
        let q_emb = vec![1.0f32, 0.0, 0.0];
        let embs = vec![
            vec![0.0f32, 1.0, 0.0], // orthogonal
            vec![1.0f32, 0.0, 0.0], // parallel
            vec![0.5f32, 0.5, 0.0], // 45 degrees
        ];
        let reranker = CosineReranker::new(q_emb, embs);
        let top = reranker.top_k(3);
        // Parallel (idx=1) should be ranked first.
        assert_eq!(top[0].0, 1, "parallel should be top-1");
    }

    #[test]
    fn rrf_deduplication_across_lists() {
        let list1 = vec![0, 1, 2];
        let list2 = vec![0, 2, 1];
        let fused = rrf_fuse(&list1, &list2, 60.0, 3);
        // doc 0 is #1 in both lists -> should be top
        assert_eq!(fused[0], 0, "fused top: {fused:?}");
    }

    // ---- context assembler integration ---------------------------------

    #[test]
    fn context_assembler_from_pipeline_results() {
        let mut p = hash_pipeline(3);
        p.ingest(
            "doc.txt",
            "chapter one content about retrieval augmented generation systems",
        )
        .unwrap();
        let passages = p.query("retrieval").unwrap();
        let texts: Vec<&str> = passages.iter().map(|p| p.text.as_str()).collect();
        let ctx = ContextAssembler::new(100_000).assemble(&texts);
        assert!(!ctx.text.is_empty());
        assert_eq!(ctx.passages_included, texts.len());
    }

    // ---- citation integration ------------------------------------------

    #[test]
    fn citation_filter_and_dedup_pipeline() {
        let triples = &[
            ("a.txt", 0.9f32, "high quality chunk"),
            ("a.txt", 0.8f32, "another chunk same source"),
            ("b.txt", 0.3f32, "low quality chunk"),
        ];
        let cits = build_citations(triples);
        let filtered = filter_by_score(cits, 0.5);
        assert_eq!(filtered.len(), 2);
        let deduped = dedup_by_source(filtered);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].source, "a.txt");
        assert!((deduped[0].score - 0.9f32).abs() < 1e-5);
    }

    #[test]
    fn sort_passages_after_rerank() {
        let passages = vec![
            ScoredPassage::new(2, "third", 0.3),
            ScoredPassage::new(0, "first", 0.9),
            ScoredPassage::new(1, "second", 0.6),
        ];
        let sorted = sort_by_score(passages);
        assert_eq!(sorted[0].index, 0);
        assert_eq!(sorted[1].index, 1);
        assert_eq!(sorted[2].index, 2);
    }
}
