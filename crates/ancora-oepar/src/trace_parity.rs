//! Trace parity module - ensures traces are structurally equal across all six language SDKs.

use std::collections::HashMap;

/// Supported language runtimes that emit traces.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go,
    Java,
    CSharp,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![
            Language::Rust,
            Language::Python,
            Language::TypeScript,
            Language::Go,
            Language::Java,
            Language::CSharp,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::TypeScript => "typescript",
            Language::Go => "go",
            Language::Java => "java",
            Language::CSharp => "csharp",
        }
    }
}

/// A span within a distributed trace.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub language: Language,
    pub attributes: HashMap<String, String>,
    pub duration_ms: u64,
}

impl Span {
    pub fn new(
        trace_id: impl Into<String>,
        span_id: impl Into<String>,
        name: impl Into<String>,
        language: Language,
    ) -> Self {
        Self {
            trace_id: trace_id.into(),
            span_id: span_id.into(),
            parent_span_id: None,
            name: name.into(),
            language,
            attributes: HashMap::new(),
            duration_ms: 0,
        }
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn with_duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_span_id = Some(parent_id.into());
        self
    }
}

/// A complete trace containing one or more spans.
#[derive(Debug, Clone)]
pub struct Trace {
    pub trace_id: String,
    pub spans: Vec<Span>,
    pub language: Language,
}

impl Trace {
    pub fn new(trace_id: impl Into<String>, language: Language) -> Self {
        Self {
            trace_id: trace_id.into(),
            spans: Vec::new(),
            language,
        }
    }

    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn span_count(&self) -> usize {
        self.spans.len()
    }
}

/// Result of a parity comparison between two traces.
#[derive(Debug, Clone)]
pub struct ParityResult {
    pub is_equal: bool,
    pub language_a: Language,
    pub language_b: Language,
    pub differences: Vec<String>,
}

impl ParityResult {
    pub fn equal(a: Language, b: Language) -> Self {
        Self {
            is_equal: true,
            language_a: a,
            language_b: b,
            differences: Vec::new(),
        }
    }

    pub fn not_equal(a: Language, b: Language, diff: Vec<String>) -> Self {
        Self {
            is_equal: false,
            language_a: a,
            language_b: b,
            differences: diff,
        }
    }
}

/// Compare two traces for structural parity.
pub fn compare_traces(a: &Trace, b: &Trace) -> ParityResult {
    let mut diffs = Vec::new();

    if a.span_count() != b.span_count() {
        diffs.push(format!(
            "span count mismatch: {} vs {}",
            a.span_count(),
            b.span_count()
        ));
    }

    for (i, (span_a, span_b)) in a.spans.iter().zip(b.spans.iter()).enumerate() {
        if span_a.name != span_b.name {
            diffs.push(format!(
                "span[{}] name mismatch: {:?} vs {:?}",
                i, span_a.name, span_b.name
            ));
        }
        for (key, val_a) in &span_a.attributes {
            match span_b.attributes.get(key) {
                None => diffs.push(format!("span[{}] missing attribute {:?} in {:?}", i, key, b.language.as_str())),
                Some(val_b) if val_a != val_b => diffs.push(format!(
                    "span[{}] attribute {:?} mismatch: {:?} vs {:?}",
                    i, key, val_a, val_b
                )),
                _ => {}
            }
        }
    }

    if diffs.is_empty() {
        ParityResult::equal(a.language.clone(), b.language.clone())
    } else {
        ParityResult::not_equal(a.language.clone(), b.language.clone(), diffs)
    }
}

/// Build a canonical reference trace for parity testing.
pub fn reference_trace(language: Language) -> Trace {
    let mut trace = Trace::new("trace-001", language.clone());
    let root = Span::new("trace-001", "span-root", "agent.run", language.clone())
        .with_attribute("gen_ai.system", "ancora")
        .with_attribute("gen_ai.operation.name", "chat")
        .with_duration(120);
    let child = Span::new("trace-001", "span-child", "agent.llm_call", language.clone())
        .with_parent("span-root")
        .with_attribute("gen_ai.system", "ancora")
        .with_attribute("gen_ai.request.model", "gpt-4o")
        .with_duration(80);
    trace.add_span(root);
    trace.add_span(child);
    trace
}
