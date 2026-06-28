//! Polyglot trace stitching - joins spans from different language SDKs into a single trace.

use std::collections::HashMap;
use crate::trace_parity::{Language, Span};

/// A polyglot trace spans multiple languages stitched by a shared trace-id.
#[derive(Debug, Clone)]
pub struct PolyglotTrace {
    pub trace_id: String,
    pub spans: Vec<Span>,
}

impl PolyglotTrace {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    /// Return the set of unique languages contributing spans.
    pub fn contributing_languages(&self) -> Vec<Language> {
        let mut seen: Vec<Language> = Vec::new();
        for span in &self.spans {
            if !seen.contains(&span.language) {
                seen.push(span.language.clone());
            }
        }
        seen
    }

    /// Look up a span by its span_id.
    pub fn find_span(&self, span_id: &str) -> Option<&Span> {
        self.spans.iter().find(|s| s.span_id == span_id)
    }

    /// Verify all parent_span_ids reference known spans.
    pub fn validate_parent_links(&self) -> Vec<String> {
        let ids: std::collections::HashSet<&str> =
            self.spans.iter().map(|s| s.span_id.as_str()).collect();
        self.spans
            .iter()
            .filter_map(|s| {
                s.parent_span_id.as_ref().and_then(|p| {
                    if ids.contains(p.as_str()) {
                        None
                    } else {
                        Some(format!(
                            "span {:?} has unknown parent {:?}",
                            s.span_id, p
                        ))
                    }
                })
            })
            .collect()
    }
}

/// A2A (agent-to-agent) call context propagated across language boundaries.
#[derive(Debug, Clone)]
pub struct A2AContext {
    pub trace_id: String,
    pub parent_span_id: String,
    pub baggage: HashMap<String, String>,
}

impl A2AContext {
    pub fn new(trace_id: impl Into<String>, parent_span_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            parent_span_id: parent_span_id.into(),
            baggage: HashMap::new(),
        }
    }

    pub fn with_baggage(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.baggage.insert(key.into(), value.into());
        self
    }
}

/// Stitch spans from multiple language agents into one polyglot trace.
pub fn stitch_polyglot_trace(
    trace_id: impl Into<String> + Clone,
    contributions: &[(Language, Vec<Span>)],
) -> PolyglotTrace {
    let mut trace = PolyglotTrace::new(trace_id);
    for (_, spans) in contributions {
        for span in spans {
            trace.add_span(span.clone());
        }
    }
    trace
}

/// Build a reference polyglot trace for testing (3 languages, 6 spans total).
pub fn reference_polyglot_trace() -> PolyglotTrace {
    let tid = "trace-poly-001";

    let rust_root = Span::new(tid, "span-rust-1", "rust.orchestrate", Language::Rust)
        .with_attribute("gen_ai.system", "ancora");

    let py_span = Span::new(tid, "span-py-1", "python.tool_call", Language::Python)
        .with_parent("span-rust-1")
        .with_attribute("gen_ai.system", "ancora");

    let ts_span = Span::new(tid, "span-ts-1", "typescript.eval", Language::TypeScript)
        .with_parent("span-rust-1")
        .with_attribute("gen_ai.system", "ancora");

    let go_span = Span::new(tid, "span-go-1", "go.grader", Language::Go)
        .with_parent("span-py-1")
        .with_attribute("gen_ai.system", "ancora");

    let java_span = Span::new(tid, "span-java-1", "java.export", Language::Java)
        .with_parent("span-ts-1")
        .with_attribute("gen_ai.system", "ancora");

    let cs_span = Span::new(tid, "span-cs-1", "csharp.feedback", Language::CSharp)
        .with_parent("span-rust-1")
        .with_attribute("gen_ai.system", "ancora");

    stitch_polyglot_trace(
        tid,
        &[
            (Language::Rust, vec![rust_root]),
            (Language::Python, vec![py_span]),
            (Language::TypeScript, vec![ts_span]),
            (Language::Go, vec![go_span]),
            (Language::Java, vec![java_span]),
            (Language::CSharp, vec![cs_span]),
        ],
    )
}

/// Shared eval dataset identifier used across all language SDKs.
pub const SHARED_EVAL_DATASET_ID: &str = "ancora-oepar-v1";

/// The canonical list of eval case IDs shared by all SDKs.
pub fn shared_eval_case_ids() -> Vec<&'static str> {
    vec!["case-001", "case-002", "case-003"]
}
