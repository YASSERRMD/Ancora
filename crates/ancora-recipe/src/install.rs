use crate::format::Recipe;
use std::collections::HashMap;

/// Installation target for a recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallTarget {
    /// A directory path on the local filesystem (no I/O performed; paths recorded only).
    Directory(String),
}

/// Status of an install operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStatus {
    Installed,
    AlreadyPresent,
    Failed(String),
}

/// A record of an installed recipe.
#[derive(Debug, Clone)]
pub struct InstalledRecipe {
    pub recipe_id: String,
    pub target: InstallTarget,
    pub status: InstallStatus,
}

/// In-memory install registry (no filesystem I/O).
#[derive(Debug, Default)]
pub struct InstallRegistry {
    entries: HashMap<String, InstalledRecipe>,
}

impl InstallRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Install a recipe into the registry under the given target.
    pub fn install(&mut self, recipe: &Recipe, target: InstallTarget) -> &InstalledRecipe {
        let key = format!("{}::{}", recipe.id, match &target { InstallTarget::Directory(d) => d });
        let status = if self.entries.contains_key(&key) {
            InstallStatus::AlreadyPresent
        } else {
            InstallStatus::Installed
        };
        let entry = InstalledRecipe {
            recipe_id: recipe.id.clone(),
            target,
            status,
        };
        self.entries.insert(key.clone(), entry);
        self.entries.get(&key).unwrap()
    }

    /// Check whether a recipe is installed in a given directory.
    pub fn is_installed(&self, recipe_id: &str, dir: &str) -> bool {
        let key = format!("{}::{}", recipe_id, dir);
        matches!(
            self.entries.get(&key).map(|e| &e.status),
            Some(InstallStatus::Installed) | Some(InstallStatus::AlreadyPresent)
        )
    }

    /// List all installed recipe IDs.
    pub fn installed_ids(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
}

/// Command-line install request.
#[derive(Debug, Clone)]
pub struct InstallCommand {
    pub recipe_id: String,
    pub target_dir: String,
}

impl InstallCommand {
    pub fn new(recipe_id: impl Into<String>, target_dir: impl Into<String>) -> Self {
        Self {
            recipe_id: recipe_id.into(),
            target_dir: target_dir.into(),
        }
    }

    /// Parse an install command from "recipe-id:target-dir" notation.
    pub fn parse(s: &str) -> Result<Self, String> {
        let mut parts = s.splitn(2, ':');
        let recipe_id = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "missing recipe-id".to_string())?;
        let target_dir = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "missing target-dir".to_string())?;
        Ok(Self::new(recipe_id, target_dir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::Recipe;
    use crate::format::RecipeStep;
    use crate::format::StepAction;

    fn make_recipe(id: &str) -> Recipe {
        let mut r = Recipe::new(id, "Test", "test recipe");
        r.add_step(RecipeStep::new("s1", StepAction::Generate, "do something"));
        r
    }

    #[test]
    fn install_and_query() {
        let mut reg = InstallRegistry::new();
        let r = make_recipe("rag-citations");
        reg.install(&r, InstallTarget::Directory("/tmp/proj".into()));
        assert!(reg.is_installed("rag-citations", "/tmp/proj"));
        assert!(!reg.is_installed("rag-citations", "/other"));
    }

    #[test]
    fn double_install_is_already_present() {
        let mut reg = InstallRegistry::new();
        let r = make_recipe("code-review");
        reg.install(&r, InstallTarget::Directory("/proj".into()));
        let entry = reg.install(&r, InstallTarget::Directory("/proj".into()));
        assert_eq!(entry.status, InstallStatus::AlreadyPresent);
    }

    #[test]
    fn parse_install_command() {
        let cmd = InstallCommand::parse("rag-citations:/my/project").unwrap();
        assert_eq!(cmd.recipe_id, "rag-citations");
        assert_eq!(cmd.target_dir, "/my/project");
    }

    #[test]
    fn parse_install_command_err() {
        assert!(InstallCommand::parse(":only-dir").is_err());
        assert!(InstallCommand::parse("only-id").is_err());
    }
}
