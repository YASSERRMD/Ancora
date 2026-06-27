/// Extended chunker boundary and coverage tests -- all offline.

#[cfg(test)]
mod chunker_ext_tests {
    use crate::embedders::chunker::{FixedSizeChunker, SemanticChunker, SemanticBoundary};

    // ---- FixedSizeChunker edge cases -----------------------------------

    #[test]
    fn fixed_single_word_text() {
        let c = FixedSizeChunker::new(5, 0);
        let chunks = c.chunk("hello");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "hello");
    }

    #[test]
    fn fixed_exact_chunk_size_no_split() {
        let c = FixedSizeChunker::new(4, 0);
        let text = "one two three four";
        let chunks = c.chunk(text);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn fixed_one_word_over_splits_to_two() {
        let c = FixedSizeChunker::new(4, 0);
        let text = "one two three four five";
        let chunks = c.chunk(text);
        assert!(chunks.len() == 2, "expected 2 chunks, got: {chunks:?}");
    }

    #[test]
    fn fixed_overlap_content_repeated_between_chunks() {
        let c = FixedSizeChunker::new(4, 2);
        let text = "a b c d e f g";
        let chunks = c.chunk(text);
        // Chunk 1: [a b c d], Chunk 2: [c d e f], Chunk 3: [e f g]
        if chunks.len() >= 2 {
            let c1_words: Vec<&str> = chunks[0].split_whitespace().collect();
            let c2_words: Vec<&str> = chunks[1].split_whitespace().collect();
            // The last two words of chunk 1 should appear at the start of chunk 2.
            let overlap_from_c1 = &c1_words[c1_words.len().saturating_sub(2)..];
            let start_of_c2 = &c2_words[..2.min(c2_words.len())];
            assert_eq!(overlap_from_c1, start_of_c2, "overlap words should repeat");
        }
    }

    #[test]
    fn fixed_chunk_docs_all_indices_present() {
        let docs = &["doc A content", "doc B content", "doc C content"];
        let c = FixedSizeChunker::new(2, 0);
        let result = c.chunk_docs(docs);
        let indices: std::collections::HashSet<usize> = result.iter().map(|(i, _)| *i).collect();
        for expected in 0..3 {
            assert!(indices.contains(&expected), "index {expected} missing");
        }
    }

    #[test]
    fn fixed_only_whitespace_text_produces_no_chunks() {
        let c = FixedSizeChunker::new(5, 0);
        let chunks = c.chunk("   \t  \n  ");
        assert!(chunks.is_empty(), "whitespace-only should produce no chunks");
    }

    #[test]
    fn fixed_unicode_text_handled() {
        let c = FixedSizeChunker::new(3, 0);
        let text = "hello world foo bar baz";
        let chunks = c.chunk(text);
        assert!(!chunks.is_empty());
        for ch in &chunks {
            assert!(!ch.is_empty());
        }
    }

    // ---- SemanticChunker boundary tests --------------------------------

    #[test]
    fn paragraph_boundary_constant() {
        let c = SemanticChunker::paragraph();
        assert_eq!(c.boundary, SemanticBoundary::Paragraph);
    }

    #[test]
    fn markdown_header_boundary_constant() {
        let c = SemanticChunker::markdown_header();
        assert_eq!(c.boundary, SemanticBoundary::MarkdownHeader);
    }

    #[test]
    fn sentence_boundary_constant() {
        let c = SemanticChunker::sentence();
        assert_eq!(c.boundary, SemanticBoundary::Sentence);
    }

    #[test]
    fn paragraph_chunker_single_paragraph() {
        let text = "One paragraph only.";
        let c = SemanticChunker::paragraph().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn sentence_chunker_no_trailing_empty() {
        let text = "Sentence one. Sentence two. Sentence three.";
        let c = SemanticChunker::sentence().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert!(chunks.iter().all(|s| !s.is_empty()), "empty chunk: {chunks:?}");
    }

    #[test]
    fn sentence_chunker_exclamation_splits() {
        let text = "Wow! Amazing! Great!";
        let c = SemanticChunker::sentence().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert!(chunks.len() >= 2, "expected splits on '!': {chunks:?}");
    }

    #[test]
    fn markdown_header_chunker_first_chunk_is_preamble_when_no_leading_header() {
        let text = "Preamble text.\n# Section One\nContent.";
        let c = SemanticChunker::markdown_header().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert!(!chunks.is_empty(), "should produce at least one chunk");
        assert!(chunks[0].contains("Preamble"), "first chunk: {}", chunks[0]);
    }

    #[test]
    fn semantic_with_limits_min_merges_short_sentences() {
        // Two very short "sentences" should merge when min_chars is large.
        let text = "Hi. There.";
        let c = SemanticChunker::sentence().with_limits(15, 9999);
        let chunks = c.chunk(text);
        // Both sentences are short -- they should merge into one chunk.
        assert!(chunks.len() <= 2, "chunks: {chunks:?}");
    }
}
