/// Recipe format version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeVersion {
    V1,
}

impl RecipeVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecipeVersion::V1 => "1",
        }
    }
}

/// A workflow recipe descriptor.
#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: RecipeVersion,
    pub steps: Vec<RecipeStep>,
}

/// A single step within a recipe.
#[derive(Debug, Clone)]
pub struct RecipeStep {
    pub name: String,
    pub action: StepAction,
    pub description: String,
}

/// The action performed by a step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepAction {
    Retrieve,
    Generate,
    Review,
    Extract,
    Classify,
    Summarize,
    Debate,
    Install,
    Custom(String),
}

impl Recipe {
    /// Create a new recipe.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            version: RecipeVersion::V1,
            steps: Vec::new(),
        }
    }

    /// Append a step to this recipe.
    pub fn add_step(&mut self, step: RecipeStep) {
        self.steps.push(step);
    }

    /// Return the number of steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Validate that the recipe has at least one step and a non-empty id.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("recipe id must not be empty".to_string());
        }
        if self.steps.is_empty() {
            return Err(format!("recipe '{}' has no steps", self.id));
        }
        Ok(())
    }
}

impl RecipeStep {
    pub fn new(
        name: impl Into<String>,
        action: StepAction,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            action,
            description: description.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_recipe_fails_validation() {
        let r = Recipe::new("", "Empty", "no steps");
        assert!(r.validate().is_err());
    }

    #[test]
    fn valid_recipe_passes_validation() {
        let mut r = Recipe::new("test", "Test Recipe", "a test");
        r.add_step(RecipeStep::new(
            "step1",
            StepAction::Generate,
            "generate something",
        ));
        assert!(r.validate().is_ok());
        assert_eq!(r.step_count(), 1);
    }
}
