//! Workflow recipes: pre-built patterns for common extension scenarios.
//!
//! Each recipe is a named, parameterised template that plugin authors
//! can instantiate to quickly set up a working workflow.

use std::collections::HashMap;

/// A parameter accepted by a recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeParam {
    pub key: &'static str,
    pub description: &'static str,
    pub required: bool,
}

/// A workflow recipe template.
#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: &'static str,
    pub description: &'static str,
    pub params: Vec<RecipeParam>,
}

impl Recipe {
    /// Validate that all required parameters are present.
    pub fn validate(&self, provided: &HashMap<&str, &str>) -> Result<(), String> {
        for param in &self.params {
            if param.required && !provided.contains_key(param.key) {
                return Err(format!("missing required param '{}'", param.key));
            }
        }
        Ok(())
    }
}

/// Returns the built-in workflow recipes.
pub fn builtin_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            name: "http-fetch",
            description: "Fetch a URL and pipe the response body into the next node",
            params: vec![
                RecipeParam { key: "url", description: "Target URL", required: true },
                RecipeParam { key: "timeout_secs", description: "Request timeout", required: false },
            ],
        },
        Recipe {
            name: "file-transform",
            description: "Read a file, apply a transformation, and write the result",
            params: vec![
                RecipeParam { key: "input_path", description: "Path to input file", required: true },
                RecipeParam { key: "output_path", description: "Path to output file", required: true },
            ],
        },
        Recipe {
            name: "parallel-fanout",
            description: "Fan a single input out to N parallel branches",
            params: vec![
                RecipeParam { key: "branches", description: "Comma-separated branch names", required: true },
            ],
        },
    ]
}

/// Look up a built-in recipe by name.
pub fn find_recipe(name: &str) -> Option<Recipe> {
    builtin_recipes().into_iter().find(|r| r.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_recipes_have_names() {
        for r in builtin_recipes() {
            assert!(!r.name.is_empty());
        }
    }

    #[test]
    fn http_fetch_requires_url() {
        let r = find_recipe("http-fetch").unwrap();
        let empty: HashMap<&str, &str> = HashMap::new();
        assert!(r.validate(&empty).is_err());
    }

    #[test]
    fn http_fetch_with_url_passes() {
        let r = find_recipe("http-fetch").unwrap();
        let mut params = HashMap::new();
        params.insert("url", "https://example.com");
        assert!(r.validate(&params).is_ok());
    }

    #[test]
    fn unknown_recipe_returns_none() {
        assert!(find_recipe("does-not-exist").is_none());
    }
}
