use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a research-and-report recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let topic = params.get("topic").unwrap_or("general topic");
    let sections: usize = params
        .get("sections")
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    let mut r = Recipe::new(
        "research-report",
        "Research and Report",
        format!(
            "Multi-step research and report generation on '{}' with {} sections.",
            topic, sections
        ),
    );

    r.add_step(RecipeStep::new(
        "outline",
        StepAction::Generate,
        format!("Generate outline with {} sections for topic '{}'", sections, topic),
    ));
    r.add_step(RecipeStep::new(
        "research",
        StepAction::Retrieve,
        "Retrieve supporting evidence for each section",
    ));
    r.add_step(RecipeStep::new(
        "draft",
        StepAction::Generate,
        "Draft report sections grounded in evidence",
    ));
    r.add_step(RecipeStep::new(
        "review",
        StepAction::Review,
        "Review draft for factual accuracy and completeness",
    ));
    r
}

/// A simple report outline.
#[derive(Debug, Clone)]
pub struct ReportOutline {
    pub title: String,
    pub sections: Vec<String>,
}

impl ReportOutline {
    pub fn new(title: impl Into<String>, sections: Vec<String>) -> Self {
        Self {
            title: title.into(),
            sections,
        }
    }

    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}

/// Generate a placeholder outline (offline, no LLM call).
pub fn generate_outline(topic: &str, n: usize) -> ReportOutline {
    let sections = (1..=n)
        .map(|i| format!("Section {}: aspect {} of {}", i, i, topic))
        .collect();
    ReportOutline::new(format!("Report on {}", topic), sections)
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
    fn outline_section_count() {
        let outline = generate_outline("AI Safety", 4);
        assert_eq!(outline.section_count(), 4);
    }
}
