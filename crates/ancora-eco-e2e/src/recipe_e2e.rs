/// Recipe end-to-end: recipe definition, installation, and execution.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RecipeStep {
    pub name: String,
    pub plugin: String,
    pub command: String,
    pub args: Vec<String>,
}

impl RecipeStep {
    pub fn new(name: &str, plugin: &str, command: &str, args: Vec<&str>) -> Self {
        RecipeStep {
            name: name.to_string(),
            plugin: plugin.to_string(),
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub version: String,
    pub steps: Vec<RecipeStep>,
}

impl Recipe {
    pub fn new(name: &str, version: &str) -> Self {
        Recipe {
            name: name.to_string(),
            version: version.to_string(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: RecipeStep) {
        self.steps.push(step);
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    Ok(String),
    Err(String),
    Skipped,
}

#[derive(Debug, Default)]
pub struct RecipeRunner {
    installed: HashMap<String, Recipe>,
}

impl RecipeRunner {
    pub fn new() -> Self {
        RecipeRunner {
            installed: HashMap::new(),
        }
    }

    pub fn install(&mut self, recipe: Recipe) -> Result<(), String> {
        if recipe.name.is_empty() {
            return Err("recipe name must not be empty".to_string());
        }
        if self.installed.contains_key(&recipe.name) {
            return Err(format!("recipe '{}' already installed", recipe.name));
        }
        self.installed.insert(recipe.name.clone(), recipe);
        Ok(())
    }

    pub fn run(&self, name: &str) -> Result<Vec<StepResult>, String> {
        let recipe = self
            .installed
            .get(name)
            .ok_or_else(|| format!("recipe '{}' not installed", name))?;
        let results = recipe
            .steps
            .iter()
            .map(|step| StepResult::Ok(format!("ran {}.{}", step.plugin, step.command)))
            .collect();
        Ok(results)
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    pub fn uninstall(&mut self, name: &str) -> bool {
        self.installed.remove(name).is_some()
    }
}
