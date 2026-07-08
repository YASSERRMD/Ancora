/// Coding assistant application.
///
/// Provides offline code snippet lookup, pattern matching, and
/// simple code generation stubs.

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Rust,
    Python,
    Go,
    TypeScript,
    Other(String),
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Python => write!(f, "python"),
            Language::Go => write!(f, "go"),
            Language::TypeScript => write!(f, "typescript"),
            Language::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snippet {
    pub id: String,
    pub language: Language,
    pub description: String,
    pub code: String,
}

impl Snippet {
    pub fn new(
        id: impl Into<String>,
        language: Language,
        description: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            language,
            description: description.into(),
            code: code.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct SnippetLibrary {
    snippets: Vec<Snippet>,
}

impl SnippetLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, snippet: Snippet) {
        self.snippets.push(snippet);
    }

    pub fn search(&self, query: &str, lang: Option<&Language>) -> Vec<&Snippet> {
        let q = query.to_lowercase();
        self.snippets
            .iter()
            .filter(|s| {
                let lang_match = lang.map_or(true, |l| &s.language == l);
                let text_match =
                    s.description.to_lowercase().contains(&q) || s.code.to_lowercase().contains(&q);
                lang_match && text_match
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct CodeSuggestion {
    pub snippets: Vec<String>,
    pub notes: Vec<String>,
}

pub struct CodingAssistant {
    library: SnippetLibrary,
}

impl CodingAssistant {
    pub fn new(library: SnippetLibrary) -> Self {
        Self { library }
    }

    pub fn suggest(&self, query: &str, lang: Option<&Language>) -> CodeSuggestion {
        let hits = self.library.search(query, lang);
        let snippets = hits.iter().map(|s| s.code.clone()).collect();
        let notes = hits.iter().map(|s| s.description.clone()).collect();
        CodeSuggestion { snippets, notes }
    }

    /// Generate a boilerplate struct stub for the given name.
    pub fn generate_struct_stub(&self, name: &str) -> String {
        format!("pub struct {} {{\n    // TODO: add fields\n}}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snippet_lookup_by_lang() {
        let mut lib = SnippetLibrary::new();
        lib.add(Snippet::new(
            "s1",
            Language::Rust,
            "read a file",
            "std::fs::read_to_string(path)?",
        ));
        let assistant = CodingAssistant::new(lib);
        let suggestion = assistant.suggest("file", Some(&Language::Rust));
        assert!(!suggestion.snippets.is_empty());
    }

    #[test]
    fn struct_stub_generation() {
        let assistant = CodingAssistant::new(SnippetLibrary::new());
        let stub = assistant.generate_struct_stub("Config");
        assert!(stub.contains("Config"));
    }
}
