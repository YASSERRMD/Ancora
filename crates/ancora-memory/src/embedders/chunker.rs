/// Text chunkers for the retrieval pipeline.
///
/// Two chunking strategies are provided:
/// - `FixedSizeChunker` -- splits text into windows of `chunk_size` tokens
///   (whitespace-delimited words) with optional `overlap` tokens.
/// - `SemanticChunker` -- splits on structural boundaries (paragraphs,
///   headers, sentences) for more coherent chunks.

// ---- fixed-size chunker ------------------------------------------------

#[derive(Debug, Clone)]
pub struct FixedSizeChunker {
    /// Window size in words.
    pub chunk_size: usize,
    /// Overlap in words between adjacent chunks.
    pub overlap: usize,
}

impl FixedSizeChunker {
    pub fn new(chunk_size: usize, overlap: usize) -> Self {
        let overlap = overlap.min(chunk_size.saturating_sub(1));
        Self { chunk_size: chunk_size.max(1), overlap }
    }

    /// Split `text` into overlapping word-window chunks.
    pub fn chunk<'a>(&self, text: &'a str) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return vec![];
        }
        let step = self.chunk_size.saturating_sub(self.overlap).max(1);
        let mut chunks = Vec::new();
        let mut start = 0usize;
        while start < words.len() {
            let end = (start + self.chunk_size).min(words.len());
            chunks.push(words[start..end].join(" "));
            if end == words.len() { break; }
            start += step;
        }
        chunks
    }

    /// Chunk multiple documents and return `(doc_index, chunk_text)` pairs.
    pub fn chunk_docs<'a>(&self, docs: &'a [&'a str]) -> Vec<(usize, String)> {
        docs.iter().enumerate().flat_map(|(i, doc)| {
            self.chunk(doc).into_iter().map(move |c| (i, c))
        }).collect()
    }
}

// ---- semantic chunker --------------------------------------------------

/// Strategy for semantic boundary detection.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticBoundary {
    /// Split on blank lines (Markdown paragraphs).
    Paragraph,
    /// Split on lines starting with `#` (Markdown headers).
    MarkdownHeader,
    /// Split on sentence-ending punctuation (`.`, `!`, `?`).
    Sentence,
}

#[derive(Debug, Clone)]
pub struct SemanticChunker {
    pub boundary: SemanticBoundary,
    /// Minimum chunk length in characters (shorter chunks are merged with next).
    pub min_chars: usize,
    /// Maximum chunk length in characters (longer chunks are split further).
    pub max_chars: usize,
}

impl SemanticChunker {
    pub fn paragraph() -> Self {
        Self { boundary: SemanticBoundary::Paragraph, min_chars: 50, max_chars: 2000 }
    }

    pub fn markdown_header() -> Self {
        Self { boundary: SemanticBoundary::MarkdownHeader, min_chars: 50, max_chars: 3000 }
    }

    pub fn sentence() -> Self {
        Self { boundary: SemanticBoundary::Sentence, min_chars: 20, max_chars: 500 }
    }

    pub fn with_limits(mut self, min_chars: usize, max_chars: usize) -> Self {
        self.min_chars = min_chars;
        self.max_chars = max_chars;
        self
    }

    pub fn chunk(&self, text: &str) -> Vec<String> {
        let raw = match self.boundary {
            SemanticBoundary::Paragraph => split_paragraphs(text),
            SemanticBoundary::MarkdownHeader => split_markdown_headers(text),
            SemanticBoundary::Sentence => split_sentences(text),
        };
        merge_and_trim(raw, self.min_chars, self.max_chars)
    }
}

fn split_paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect()
}

fn split_markdown_headers(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        if line.starts_with('#') && !current.is_empty() {
            chunks.push(current.trim().to_owned());
            current = String::new();
        }
        if !current.is_empty() { current.push('\n'); }
        current.push_str(line);
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_owned());
    }
    chunks
}

fn split_sentences(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?') {
            let trimmed = current.trim().to_owned();
            if !trimmed.is_empty() {
                chunks.push(trimmed);
            }
            current.clear();
        }
    }
    let remaining = current.trim().to_owned();
    if !remaining.is_empty() {
        chunks.push(remaining);
    }
    chunks
}

fn merge_and_trim(mut chunks: Vec<String>, min_chars: usize, max_chars: usize) -> Vec<String> {
    // Merge short chunks with their successor.
    let mut merged: Vec<String> = Vec::new();
    let mut carry = String::new();
    for chunk in chunks.drain(..) {
        if carry.len() + chunk.len() < min_chars {
            if !carry.is_empty() { carry.push(' '); }
            carry.push_str(&chunk);
        } else {
            if !carry.is_empty() {
                carry.push(' ');
                carry.push_str(&chunk);
                merged.push(carry.trim().to_owned());
                carry = String::new();
            } else {
                merged.push(chunk.trim().to_owned());
            }
        }
    }
    if !carry.is_empty() {
        merged.push(carry.trim().to_owned());
    }
    // Trim over-long chunks to max_chars at word boundaries.
    merged.into_iter().flat_map(|chunk| {
        if chunk.len() <= max_chars {
            vec![chunk]
        } else {
            let words: Vec<&str> = chunk.split_whitespace().collect();
            let mut sub_chunks = Vec::new();
            let mut current = String::new();
            for word in words {
                if current.len() + word.len() + 1 > max_chars && !current.is_empty() {
                    sub_chunks.push(current.trim().to_owned());
                    current = word.to_owned();
                } else {
                    if !current.is_empty() { current.push(' '); }
                    current.push_str(word);
                }
            }
            if !current.trim().is_empty() {
                sub_chunks.push(current.trim().to_owned());
            }
            sub_chunks
        }
    }).filter(|s| !s.is_empty()).collect()
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod chunker_tests {
    use super::*;

    // ---- fixed-size tests ------------------------------------------------

    #[test]
    fn fixed_single_chunk_when_text_fits() {
        let c = FixedSizeChunker::new(10, 0);
        let text = "one two three four";
        let chunks = c.chunk(text);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn fixed_splits_into_multiple_chunks() {
        let c = FixedSizeChunker::new(3, 0);
        let text = "a b c d e f g";  // 7 words
        let chunks = c.chunk(text);
        assert!(chunks.len() > 1, "chunks: {chunks:?}");
    }

    #[test]
    fn fixed_overlap_produces_more_chunks() {
        let words = (0..10).map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
        let c_no_overlap = FixedSizeChunker::new(5, 0);
        let c_overlap = FixedSizeChunker::new(5, 2);
        let n0 = c_no_overlap.chunk(&words).len();
        let n2 = c_overlap.chunk(&words).len();
        assert!(n2 >= n0, "overlap should produce at least as many chunks");
    }

    #[test]
    fn fixed_each_chunk_at_most_chunk_size_words() {
        let c = FixedSizeChunker::new(4, 1);
        let text = "w0 w1 w2 w3 w4 w5 w6 w7 w8 w9";
        for chunk in c.chunk(text) {
            let word_count = chunk.split_whitespace().count();
            assert!(word_count <= 4, "chunk has {word_count} words: {chunk}");
        }
    }

    #[test]
    fn fixed_empty_text_produces_no_chunks() {
        let c = FixedSizeChunker::new(5, 0);
        assert!(c.chunk("").is_empty());
    }

    #[test]
    fn fixed_chunk_docs_includes_doc_index() {
        let docs = &["hello world", "foo bar baz"];
        let c = FixedSizeChunker::new(2, 0);
        let result = c.chunk_docs(docs);
        assert!(result.iter().any(|(i, _)| *i == 0));
        assert!(result.iter().any(|(i, _)| *i == 1));
    }

    #[test]
    fn fixed_overlap_cap_prevents_invalid_state() {
        // overlap >= chunk_size is clamped
        let c = FixedSizeChunker::new(3, 5);
        assert!(c.overlap < c.chunk_size);
    }

    // ---- semantic tests --------------------------------------------------

    #[test]
    fn paragraph_chunker_splits_on_blank_lines() {
        let text = "First paragraph here.\n\nSecond paragraph here.";
        let c = SemanticChunker::paragraph().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert_eq!(chunks.len(), 2, "chunks: {chunks:?}");
    }

    #[test]
    fn markdown_header_chunker_splits_on_headers() {
        let text = "# Section One\nContent one.\n# Section Two\nContent two.";
        let c = SemanticChunker::markdown_header().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert_eq!(chunks.len(), 2, "chunks: {chunks:?}");
    }

    #[test]
    fn sentence_chunker_splits_on_period() {
        let text = "Hello world. This is a test. Goodbye.";
        let c = SemanticChunker::sentence().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert!(chunks.len() >= 2, "chunks: {chunks:?}");
    }

    #[test]
    fn semantic_chunker_no_empty_chunks() {
        let text = "One.\n\n\n\nTwo.\n\nThree.";
        let c = SemanticChunker::paragraph().with_limits(1, 9999);
        let chunks = c.chunk(text);
        assert!(chunks.iter().all(|s| !s.is_empty()), "empty chunk found in: {chunks:?}");
    }

    #[test]
    fn semantic_max_chars_splits_long_chunks() {
        let long = "word ".repeat(300);
        let c = SemanticChunker::paragraph().with_limits(1, 100);
        let chunks = c.chunk(long.trim());
        assert!(chunks.iter().all(|ch| ch.len() <= 120), "found long chunk");
    }
}
