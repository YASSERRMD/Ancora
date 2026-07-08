//! Offline tests confirming the Embedder trait is satisfied by local embedders
//! and that all helper functions behave correctly across edge cases.

#[cfg(test)]
mod embedder_trait_offline_tests {
    use crate::embedders::cohere::{CohereConfig, CohereEmbedder, CohereReranker};
    use crate::embedders::embedder::{
        cosine_similarity, l2_normalize, parse_openai_batch_embeddings, parse_openai_embedding,
        EmbedError, Embedder, Reranker,
    };
    use crate::embedders::local::HashEmbedder;
    use crate::embedders::openai::OpenAiEmbedConfig;
    use crate::embedders::openai::OpenAiEmbedder;
    use crate::embedders::qwen_glm::{QwenEmbedConfig, QwenEmbedder};
    use std::sync::Arc;

    // ---- Embedder trait satisfaction ------------------------------------

    #[test]
    fn hash_embedder_satisfies_embedder_trait() {
        let e: Arc<dyn Embedder> = Arc::new(HashEmbedder::new(32));
        let v = e.embed("hello world").unwrap();
        assert_eq!(v.len(), 32);
    }

    #[test]
    fn openai_embedder_satisfies_embedder_trait() {
        let cfg = OpenAiEmbedConfig::new("key", "model").with_dimensions(16);
        let e: Arc<dyn Embedder> = Arc::new(OpenAiEmbedder::new(cfg, 16));
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 16);
    }

    #[test]
    fn cohere_embedder_satisfies_embedder_trait() {
        let cfg = CohereConfig::new("key");
        let e: Arc<dyn Embedder> = Arc::new(CohereEmbedder::new(cfg, 8));
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 8);
    }

    #[test]
    fn qwen_embedder_satisfies_embedder_trait() {
        let cfg = QwenEmbedConfig::new("key");
        let e: Arc<dyn Embedder> = Arc::new(QwenEmbedder::new(cfg, 12));
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 12);
    }

    // ---- Reranker trait satisfaction ------------------------------------

    #[test]
    fn cohere_reranker_satisfies_reranker_trait() {
        let cfg = CohereConfig::new("key");
        let r: Arc<dyn Reranker> = Arc::new(CohereReranker::new(cfg));
        let scores = r.rerank("query", &["doc1", "doc2"]).unwrap();
        assert_eq!(scores.len(), 2);
    }

    // ---- batch default implementation -----------------------------------

    #[test]
    fn hash_embedder_batch_default_impl() {
        let e = HashEmbedder::new(8);
        let results = e.embed_batch(&["a", "b", "c", "d"]).unwrap();
        assert_eq!(results.len(), 4);
        assert!(results.iter().all(|v| v.len() == 8));
    }

    // ---- cosine similarity edge cases -----------------------------------

    #[test]
    fn cosine_similarity_zero_first_vec() {
        let a = vec![0.0f32; 4];
        let b = vec![1.0f32, 0.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn cosine_similarity_antiparallel() {
        let a = [1.0f32, 0.0];
        let b = [-1.0f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim + 1.0).abs() < 1e-5, "sim: {sim}");
    }

    #[test]
    fn cosine_similarity_45_degree() {
        let a = [1.0f32, 1.0];
        let b = [1.0f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        let expected = 1.0f32 / std::f32::consts::SQRT_2;
        assert!(
            (sim - expected).abs() < 1e-5,
            "sim: {sim} expected: {expected}"
        );
    }

    // ---- l2_normalize edge cases ----------------------------------------

    #[test]
    fn l2_normalize_already_normalized() {
        let mut v = vec![1.0f32, 0.0, 0.0];
        l2_normalize(&mut v);
        assert!((v[0] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn l2_normalize_large_vector() {
        let mut v: Vec<f32> = (1..=100).map(|i| i as f32).collect();
        l2_normalize(&mut v);
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-4, "norm: {norm}");
    }

    // ---- error type tests -----------------------------------------------

    #[test]
    fn embed_error_display_not_empty() {
        let e = EmbedError::InputTooLong {
            max_tokens: 8192,
            got: 9000,
        };
        assert!(!e.to_string().is_empty());
    }

    #[test]
    fn embed_error_other_not_transient() {
        let e = EmbedError::Other("oops".to_owned());
        assert!(!e.is_transient());
    }

    #[test]
    fn embed_error_transient_variant_is_transient() {
        let e = EmbedError::Transient("server busy".to_owned());
        assert!(e.is_transient());
    }

    // ---- parse helpers --------------------------------------------------

    #[test]
    fn parse_openai_batch_empty_data_returns_empty_vec() {
        let body = serde_json::json!({ "data": [] });
        let embs = parse_openai_batch_embeddings(&body).unwrap();
        assert!(embs.is_empty());
    }

    #[test]
    fn parse_openai_embedding_malformed_values_default_to_zero() {
        let body = serde_json::json!({
            "data": [{ "embedding": [null, "bad", 0.5] }]
        });
        let emb = parse_openai_embedding(&body).unwrap();
        assert_eq!(emb[0], 0.0);
        assert_eq!(emb[1], 0.0);
        assert!((emb[2] - 0.5f32).abs() < 1e-5);
    }
}
