/// Eval helpers - run evaluations against traces across all SDK languages.
use crate::context::{EvalResult, Trace};

/// Evaluation criteria for a trace.
#[derive(Debug, Clone)]
pub struct EvalCriteria {
    pub name: String,
    pub min_span_count: usize,
    pub max_duration_ns: Option<u64>,
    pub required_span_names: Vec<String>,
}

impl EvalCriteria {
    pub fn new(name: impl Into<String>) -> Self {
        EvalCriteria {
            name: name.into(),
            min_span_count: 0,
            max_duration_ns: None,
            required_span_names: Vec::new(),
        }
    }

    pub fn with_min_spans(mut self, n: usize) -> Self {
        self.min_span_count = n;
        self
    }

    pub fn with_max_duration_ns(mut self, ns: u64) -> Self {
        self.max_duration_ns = Some(ns);
        self
    }

    pub fn with_required_span(mut self, name: impl Into<String>) -> Self {
        self.required_span_names.push(name.into());
        self
    }
}

/// Runs an evaluation of a trace against criteria.
pub struct EvalRunner;

impl EvalRunner {
    pub fn new() -> Self {
        EvalRunner
    }

    pub fn evaluate(&self, trace: &Trace, criteria: &EvalCriteria) -> EvalResult {
        let mut notes = Vec::new();
        let mut passed = true;

        // Check span count.
        if trace.span_count() < criteria.min_span_count {
            passed = false;
            notes.push(format!(
                "expected >= {} spans, got {}",
                criteria.min_span_count,
                trace.span_count()
            ));
        }

        // Check required span names.
        let span_names: Vec<&str> = trace.spans.iter().map(|s| s.name.as_str()).collect();
        for req in &criteria.required_span_names {
            if !span_names.contains(&req.as_str()) {
                passed = false;
                notes.push(format!("missing required span: {}", req));
            }
        }

        // Check root span duration.
        if let Some(max_dur) = criteria.max_duration_ns {
            if let Some(root) = trace.root_span() {
                if let Some(dur) = root.duration_ns() {
                    if dur > max_dur {
                        passed = false;
                        notes.push(format!("root duration {}ns exceeds max {}ns", dur, max_dur));
                    }
                }
            }
        }

        let score = if passed { 1.0 } else { 0.0 };
        let note_str = if notes.is_empty() {
            "all checks passed".into()
        } else {
            notes.join("; ")
        };

        EvalResult::new(trace.trace_id.clone(), passed, score, note_str)
    }
}

impl Default for EvalRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Language-tagged eval result for multi-language eval runs.
#[derive(Debug, Clone)]
pub struct LangEvalResult {
    pub language: String,
    pub result: EvalResult,
}

impl LangEvalResult {
    pub fn new(language: impl Into<String>, result: EvalResult) -> Self {
        LangEvalResult {
            language: language.into(),
            result,
        }
    }
}

/// Run evaluations for multiple language SDKs and aggregate results.
pub fn run_multilang_eval(
    traces: &[(&str, &Trace)],
    criteria: &EvalCriteria,
) -> Vec<LangEvalResult> {
    let runner = EvalRunner::new();
    traces
        .iter()
        .map(|(lang, trace)| {
            let result = runner.evaluate(trace, criteria);
            LangEvalResult::new(*lang, result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Span;

    fn make_trace(id: &str) -> Trace {
        let mut t = Trace::new(id);
        t.add_span(Span::new("s1", "agent.run", 0).finish(500));
        t.add_span(
            Span::new("s2", "llm.call", 10)
                .with_parent("s1")
                .finish(400),
        );
        t
    }

    #[test]
    fn eval_passes_with_sufficient_spans() {
        let trace = make_trace("eval-t1");
        let criteria = EvalCriteria::new("basic").with_min_spans(2);
        let runner = EvalRunner::new();
        let result = runner.evaluate(&trace, &criteria);
        assert!(result.passed);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn eval_fails_missing_required_span() {
        let trace = make_trace("eval-t2");
        let criteria = EvalCriteria::new("requires-embed").with_required_span("embed.call");
        let runner = EvalRunner::new();
        let result = runner.evaluate(&trace, &criteria);
        assert!(!result.passed);
        assert!(result.notes.contains("embed.call"));
    }

    #[test]
    fn eval_fails_duration_exceeded() {
        let trace = make_trace("eval-t3");
        let criteria = EvalCriteria::new("fast").with_max_duration_ns(100);
        let runner = EvalRunner::new();
        let result = runner.evaluate(&trace, &criteria);
        assert!(!result.passed);
        assert!(result.notes.contains("exceeds max"));
    }

    #[test]
    fn multilang_eval() {
        let t1 = make_trace("go-t1");
        let t2 = make_trace("py-t1");
        let criteria = EvalCriteria::new("basic").with_min_spans(1);
        let results = run_multilang_eval(&[("go", &t1), ("python", &t2)], &criteria);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.result.passed));
    }
}
