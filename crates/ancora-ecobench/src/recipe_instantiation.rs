//! Recipe instantiation time measurement.
//!
//! A "recipe" is a named, parameterised workflow template that can be stamped
//! out into a concrete execution plan. This module models the time cost of
//! validating template parameters and building the execution plan.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A parameter declaration in a recipe template.
#[derive(Debug, Clone)]
pub struct ParamDecl {
    /// Parameter name.
    pub name: String,
    /// Expected type tag.
    pub ty: ParamType,
    /// Whether a value must be provided.
    pub required: bool,
}

/// Simple type tags for recipe parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamType {
    String,
    Integer,
    Boolean,
    List,
}

/// A recipe template.
#[derive(Debug, Clone)]
pub struct RecipeTemplate {
    /// Unique recipe name.
    pub name: String,
    /// Ordered list of steps (just labels in this model).
    pub steps: Vec<String>,
    /// Declared parameters.
    pub params: Vec<ParamDecl>,
}

impl RecipeTemplate {
    /// Create a new, empty template.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            steps: Vec::new(),
            params: Vec::new(),
        }
    }

    /// Add a step to the template.
    pub fn with_step(mut self, step: &str) -> Self {
        self.steps.push(step.to_owned());
        self
    }

    /// Add a required string parameter.
    pub fn with_required_param(mut self, name: &str) -> Self {
        self.params.push(ParamDecl {
            name: name.to_owned(),
            ty: ParamType::String,
            required: true,
        });
        self
    }
}

/// A concrete execution plan produced from a recipe template.
#[derive(Debug)]
pub struct ExecutionPlan {
    /// Name of the originating recipe.
    pub recipe_name: String,
    /// Bound parameter values.
    pub bound_params: HashMap<String, String>,
    /// Ordered step labels.
    pub steps: Vec<String>,
}

/// Result of an instantiation attempt.
#[derive(Debug)]
pub enum InstantiateResult {
    /// Successful instantiation.
    Ok {
        plan: ExecutionPlan,
        elapsed: Duration,
    },
    /// Validation failed with a list of errors.
    Err {
        errors: Vec<String>,
        elapsed: Duration,
    },
}

/// Instantiate a recipe template with the provided parameter bindings.
pub fn instantiate(
    template: &RecipeTemplate,
    bindings: HashMap<String, String>,
) -> InstantiateResult {
    let start = Instant::now();
    let mut errors: Vec<String> = Vec::new();

    // Validate required parameters.
    for decl in &template.params {
        if decl.required && !bindings.contains_key(&decl.name) {
            errors.push(format!("missing required param: {}", decl.name));
        }
    }

    if !errors.is_empty() {
        return InstantiateResult::Err {
            errors,
            elapsed: start.elapsed(),
        };
    }

    let plan = ExecutionPlan {
        recipe_name: template.name.clone(),
        bound_params: bindings,
        steps: template.steps.clone(),
    };

    InstantiateResult::Ok {
        plan,
        elapsed: start.elapsed(),
    }
}

/// Regression threshold for recipe instantiation in microseconds.
pub const INSTANTIATE_TARGET_US: u64 = 2_000;

/// Returns `true` if instantiation completed within the regression threshold.
pub fn within_target(elapsed: Duration) -> bool {
    elapsed.as_micros() as u64 <= INSTANTIATE_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_template() -> RecipeTemplate {
        RecipeTemplate::new("greet")
            .with_step("validate")
            .with_step("execute")
            .with_required_param("recipient")
    }

    #[test]
    fn instantiation_succeeds_with_all_params() {
        let t = make_template();
        let mut b = HashMap::new();
        b.insert("recipient".to_owned(), "world".to_owned());
        let r = instantiate(&t, b);
        assert!(matches!(r, InstantiateResult::Ok { .. }));
    }

    #[test]
    fn instantiation_fails_with_missing_param() {
        let t = make_template();
        let r = instantiate(&t, HashMap::new());
        match r {
            InstantiateResult::Err { errors, .. } => {
                assert!(!errors.is_empty());
            }
            _ => panic!("expected Err"),
        }
    }
}
