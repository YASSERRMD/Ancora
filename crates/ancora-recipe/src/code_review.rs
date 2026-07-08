use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a code-review recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let lang = params.get("language").unwrap_or("any");
    let strict: bool = params.get("strict").map(|v| v == "true").unwrap_or(false);

    let mut r = Recipe::new(
        "code-review",
        "Code Review",
        format!(
            "Automated code review for {} ({} mode).",
            lang,
            if strict { "strict" } else { "standard" }
        ),
    );

    r.add_step(RecipeStep::new(
        "parse",
        StepAction::Extract,
        "Parse and tokenize the provided source files",
    ));
    r.add_step(RecipeStep::new(
        "lint",
        StepAction::Review,
        format!("Run lint analysis for {} with strict={}", lang, strict),
    ));
    r.add_step(RecipeStep::new(
        "security",
        StepAction::Review,
        "Check for common security anti-patterns",
    ));
    r.add_step(RecipeStep::new(
        "summarize",
        StepAction::Summarize,
        "Produce a structured review report with severity levels",
    ));
    r
}

/// Severity level of a review finding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// A single review finding.
#[derive(Debug, Clone)]
pub struct Finding {
    pub line: usize,
    pub severity: Severity,
    pub message: String,
}

impl Finding {
    pub fn new(line: usize, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            line,
            severity,
            message: message.into(),
        }
    }
}

/// Filter findings by minimum severity.
pub fn filter_by_severity<'a>(findings: &'a [Finding], min: &Severity) -> Vec<&'a Finding> {
    findings.iter().filter(|f| &f.severity >= min).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_has_four_steps() {
        let params = ParamSet::default();
        let r = build(&params);
        assert_eq!(r.step_count(), 4);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn filter_findings() {
        let findings = vec![
            Finding::new(1, Severity::Info, "style"),
            Finding::new(2, Severity::Warning, "unused var"),
            Finding::new(3, Severity::Error, "null deref"),
        ];
        let errors = filter_by_severity(&findings, &Severity::Error);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
    }
}
