/// Document loaders for text, Markdown, and simple PDF text extraction.
///
/// All loaders return `Vec<Document>` -- a list of documents with content
/// and metadata.  They do NOT make network calls.

use std::collections::HashMap;
use serde_json::{json, Value};

// ---- document type ------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Document {
    /// Source identifier (file path, URL, etc.).
    pub source: String,
    /// Extracted text content.
    pub text: String,
    /// Arbitrary metadata (title, page, section, etc.).
    pub metadata: Value,
}

impl Document {
    pub fn new(source: impl Into<String>, text: impl Into<String>) -> Self {
        Self { source: source.into(), text: text.into(), metadata: json!({}) }
    }

    pub fn with_metadata(mut self, meta: Value) -> Self {
        self.metadata = meta; self
    }

    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    pub fn char_count(&self) -> usize {
        self.text.len()
    }
}

// ---- text loader -------------------------------------------------------

/// Load a plain-text string as a single `Document`.
pub fn load_text(source: impl Into<String>, content: &str) -> Document {
    Document::new(source, content.trim())
}

/// Load multiple text strings as separate `Document`s.
pub fn load_texts(pairs: &[(&str, &str)]) -> Vec<Document> {
    pairs.iter().map(|(source, content)| load_text(*source, content)).collect()
}

// ---- markdown loader ---------------------------------------------------

/// Load a Markdown string as a `Document`, optionally stripping syntax.
pub fn load_markdown(source: impl Into<String>, content: &str, strip_syntax: bool) -> Document {
    let text = if strip_syntax {
        strip_markdown(content)
    } else {
        content.trim().to_owned()
    };
    Document::new(source, text)
}

/// Split a Markdown document into one `Document` per section (H1/H2).
pub fn split_markdown_sections(source: &str, content: &str) -> Vec<Document> {
    let mut docs = Vec::new();
    let mut current_title = String::from("(preamble)");
    let mut current_body = String::new();

    for line in content.lines() {
        if line.starts_with("## ") || line.starts_with("# ") {
            if !current_body.trim().is_empty() {
                docs.push(Document::new(
                    format!("{source}#{}", slug(&current_title)),
                    current_body.trim(),
                ).with_metadata(json!({ "section": current_title })));
            }
            current_title = line.trim_start_matches('#').trim().to_owned();
            current_body.clear();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    if !current_body.trim().is_empty() {
        docs.push(Document::new(
            format!("{source}#{}", slug(&current_title)),
            current_body.trim(),
        ).with_metadata(json!({ "section": current_title })));
    }
    docs
}

fn strip_markdown(md: &str) -> String {
    let mut out = String::new();
    for line in md.lines() {
        let trimmed = line.trim_start_matches('#').trim();
        // Skip fenced code blocks and inline code markers.
        if trimmed.starts_with("```") { continue; }
        let clean: String = trimmed.chars().filter(|c| *c != '*' && *c != '_').collect();
        out.push_str(&clean);
        out.push('\n');
    }
    out.trim().to_owned()
}

fn slug(s: &str) -> String {
    s.to_lowercase().chars().map(|c| if c.is_alphanumeric() { c } else { '-' }).collect()
}

// ---- pdf loader (text extraction stub) ---------------------------------

/// Simulated PDF text extraction.  In production this would use a library
/// such as `lopdf` or `pdfium-render`; here we just store the raw bytes as
/// a note and return a stub document so the pipeline can be tested.
pub struct PdfLoader {
    pub max_pages: Option<usize>,
}

impl PdfLoader {
    pub fn new() -> Self { Self { max_pages: None } }
    pub fn with_max_pages(mut self, n: usize) -> Self { self.max_pages = Some(n); self }

    /// Extract text from a PDF given as raw bytes.
    /// In tests, pass a fake byte slice; returns a stub document.
    pub fn load_bytes(&self, source: &str, _bytes: &[u8]) -> Vec<Document> {
        // Production: extract pages; here return stub.
        let page_limit = self.max_pages.unwrap_or(usize::MAX);
        (0..page_limit.min(1)).map(|page| {
            Document::new(
                format!("{source}#page-{}", page + 1),
                "[PDF text extraction placeholder]",
            ).with_metadata(json!({ "page": page + 1, "source": source }))
        }).collect()
    }
}

impl Default for PdfLoader {
    fn default() -> Self { Self::new() }
}

// ---- metadata enrichment -----------------------------------------------

/// Add or merge metadata into a list of documents.
pub fn enrich_metadata(docs: Vec<Document>, extra: &HashMap<String, Value>) -> Vec<Document> {
    docs.into_iter().map(|mut doc| {
        if let Value::Object(ref mut map) = doc.metadata {
            for (k, v) in extra {
                map.insert(k.clone(), v.clone());
            }
        }
        doc
    }).collect()
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod loader_tests {
    use super::*;

    #[test]
    fn load_text_returns_document() {
        let doc = load_text("file.txt", "Hello world");
        assert_eq!(doc.source, "file.txt");
        assert_eq!(doc.text, "Hello world");
    }

    #[test]
    fn load_text_trims_whitespace() {
        let doc = load_text("f.txt", "  hello  \n");
        assert_eq!(doc.text, "hello");
    }

    #[test]
    fn load_texts_produces_multiple_docs() {
        let pairs = &[("a.txt", "text a"), ("b.txt", "text b")];
        let docs = load_texts(pairs);
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn document_word_count() {
        let doc = Document::new("f", "one two three");
        assert_eq!(doc.word_count(), 3);
    }

    #[test]
    fn document_char_count() {
        let doc = Document::new("f", "hello");
        assert_eq!(doc.char_count(), 5);
    }

    #[test]
    fn load_markdown_preserves_content() {
        let md = "# Title\n\nParagraph content.";
        let doc = load_markdown("f.md", md, false);
        assert!(doc.text.contains("Title"), "text: {}", doc.text);
    }

    #[test]
    fn load_markdown_strip_removes_hashes() {
        let md = "# Title\n\nParagraph content.";
        let doc = load_markdown("f.md", md, true);
        assert!(!doc.text.contains('#'), "text: {}", doc.text);
    }

    #[test]
    fn split_markdown_sections_produces_one_doc_per_section() {
        let md = "# Section One\nContent A.\n## Section Two\nContent B.";
        let docs = split_markdown_sections("readme.md", md);
        assert_eq!(docs.len(), 2, "docs: {docs:?}");
    }

    #[test]
    fn split_markdown_sections_has_metadata() {
        let md = "# Intro\nHello.\n# Conclusion\nBye.";
        let docs = split_markdown_sections("f.md", md);
        assert!(docs[0].metadata["section"].as_str().is_some());
    }

    #[test]
    fn pdf_loader_stub_returns_document() {
        let loader = PdfLoader::new().with_max_pages(2);
        let docs = loader.load_bytes("report.pdf", &[0x25, 0x50, 0x44, 0x46]);
        assert!(!docs.is_empty());
        assert!(docs[0].text.contains("PDF"), "text: {}", docs[0].text);
    }

    #[test]
    fn enrich_metadata_adds_keys() {
        let docs = vec![Document::new("f", "content")];
        let mut extra = HashMap::new();
        extra.insert("lang".to_owned(), serde_json::json!("en"));
        let enriched = enrich_metadata(docs, &extra);
        assert_eq!(enriched[0].metadata["lang"], "en");
    }
}
