/// Generative search (RAG) query tests for the Weaviate backend.
/// All offline.

#[cfg(test)]
mod weaviate_generative_tests {
    use crate::backends::weaviate::*;

    #[test]
    fn single_result_query_has_prompt_in_query() {
        let body =
            graphql_generative_query("Document", &[0.1f32], 3, "Summarize {title}", &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("singleResult"), "query: {q}");
        assert!(q.contains("Summarize"), "query: {q}");
    }

    #[test]
    fn grouped_result_query_has_task_in_query() {
        let body = graphql_grouped_generative_query(
            "Document",
            &[0.1f32],
            5,
            "Compare all docs",
            &["title"],
        );
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("groupedResult"), "query: {q}");
        assert!(q.contains("Compare all docs"), "query: {q}");
    }

    #[test]
    fn generative_query_includes_error_field() {
        let body = graphql_generative_query("Document", &[0.1f32], 2, "test", &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("error"), "query: {q}");
    }

    #[test]
    fn generative_query_includes_additional_with_id() {
        let body = graphql_generative_query("Document", &[0.1f32], 2, "test", &["title"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("_additional"), "query: {q}");
        assert!(q.contains("id"), "query: {q}");
    }

    #[test]
    fn grouped_query_limit_propagates() {
        let body = graphql_grouped_generative_query("Article", &[0.1f32], 42, "task", &["body"]);
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("limit: 42"), "query: {q}");
    }

    #[test]
    fn generative_query_includes_requested_fields() {
        let body = graphql_generative_query(
            "Document",
            &[0.1f32],
            3,
            "prompt",
            &["title", "body", "year"],
        );
        let q = body["query"].as_str().unwrap();
        assert!(q.contains("title"), "query: {q}");
        assert!(q.contains("body"), "query: {q}");
        assert!(q.contains("year"), "query: {q}");
    }
}
